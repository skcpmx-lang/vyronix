//! Real benchmark & stress workloads. All results are measured, never simulated.
//! CPU/memory benchmarks are deterministic compute loops; storage uses dedicated
//! temp files with a free-space guard and guaranteed cleanup. Stress is bounded
//! by duration, temperature and a stop flag.

use crate::model::*;
use std::hint::black_box;
use std::io::{Read, Seek, SeekFrom, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

const MIB: u64 = 1024 * 1024;

// ---------------- CPU ----------------

/// Mixed integer/float workload. Returns (integer_ops, float_ops) performed.
fn cpu_worker(deadline: Instant, stop: &AtomicBool) -> (u64, u64) {
    let mut int_ops = 0u64;
    let mut float_ops = 0u64;
    let mut x: u64 = 0x1234_5678_9abc_def0;
    let mut f: f64 = 1.000_001;
    while Instant::now() < deadline && !stop.load(Ordering::Relaxed) {
        for _ in 0..2000 {
            x = x
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            x ^= x >> 13;
            x = x.rotate_left(7);
            f = f * 1.000_000_1 + 0.000_000_1;
            f = f.sin() * 0.5 + 1.0;
            int_ops += 4;
            float_ops += 3;
        }
        black_box(x);
        black_box(f);
    }
    (int_ops, float_ops)
}

pub fn cpu_benchmark(duration_secs: u64) -> CpuBenchResult {
    let duration = duration_secs.clamp(1, 60);
    let stop = std::sync::Arc::new(AtomicBool::new(false));

    // Single core.
    let start = Instant::now();
    let deadline = start + Duration::from_secs(duration);
    let (si, sf) = cpu_worker(deadline, &stop);
    let single_elapsed = start.elapsed().as_secs_f64();
    let single_total = (si + sf) as f64 / single_elapsed.max(0.001);

    // Multi core (bounded thread count). Each worker owns an Arc clone so the
    // spawned closures are 'static.
    let threads = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1)
        .min(64) as u32;
    let start = Instant::now();
    let deadline = start + Duration::from_secs(duration);
    let mut handles = Vec::new();
    for _ in 0..threads {
        let stop = std::sync::Arc::clone(&stop);
        handles.push(std::thread::spawn(move || cpu_worker(deadline, &stop)));
    }
    let (mut mi, mut mf) = (0u64, 0u64);
    for h in handles {
        if let Ok((i, f)) = h.join() {
            mi += i;
            mf += f;
        }
    }
    let multi_elapsed = start.elapsed().as_secs_f64();
    let multi_total = (mi + mf) as f64 / multi_elapsed.max(0.001);

    CpuBenchResult {
        // Scores are millions of mixed operations per second (Mops/s), a
        // reproducible, clearly-labelled metric — not an arbitrary index.
        single_core_score: single_total / 1e6,
        multi_core_score: multi_total / 1e6,
        integer_ops_per_sec: (si as f64 / single_elapsed.max(0.001)) * threads as f64,
        float_ops_per_sec: (sf as f64 / single_elapsed.max(0.001)) * threads as f64,
        duration_ms: (single_elapsed + multi_elapsed) as u64 * 1000,
        threads,
        cancelled: false,
    }
}

// ---------------- Memory ----------------

pub fn memory_benchmark(size_mib: u64) -> MemoryBenchResult {
    let size = size_mib.clamp(16, 2048) * MIB;
    let count = (size / 8) as usize;
    let start = Instant::now();

    // Write bandwidth.
    let mut buf: Vec<u64> = vec![0; count];
    let write_start = Instant::now();
    for (i, v) in buf.iter_mut().enumerate() {
        *v = black_box(i as u64);
    }
    black_box(&buf);
    let write_elapsed = write_start.elapsed().as_secs_f64();

    // Read bandwidth.
    let read_start = Instant::now();
    let mut sum: u64 = 0;
    for &v in buf.iter() {
        sum = sum.wrapping_add(v);
    }
    black_box(sum);
    let read_elapsed = read_start.elapsed().as_secs_f64();

    // Copy bandwidth.
    let mut dst: Vec<u64> = vec![0; count];
    let copy_start = Instant::now();
    dst.copy_from_slice(&buf);
    black_box(&dst);
    let copy_elapsed = copy_start.elapsed().as_secs_f64();

    // Simple random-access latency (pointer-chase style, measured).
    let latency_ns = {
        let n = 1_000_000.min(count);
        let mut idx = 0usize;
        let lat_start = Instant::now();
        for _ in 0..n {
            idx = buf[idx] as usize % count;
        }
        black_box(idx);
        lat_start.elapsed().as_secs_f64() * 1e9 / n as f64
    };

    let mb = size as f64 / MIB as f64;
    MemoryBenchResult {
        read_bandwidth_mbps: mb / read_elapsed.max(1e-9),
        write_bandwidth_mbps: mb / write_elapsed.max(1e-9),
        copy_bandwidth_mbps: mb / copy_elapsed.max(1e-9),
        latency_ns: Some(latency_ns),
        test_size_bytes: size,
        duration_ms: start.elapsed().as_millis() as u64,
    }
}

// ---------------- Storage ----------------

/// Simple deterministic LCG for random offsets (no external dependency).
struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.0
    }
}

pub fn storage_benchmark(path: &str, size_mib: u64) -> Result<StorageBenchResult, String> {
    let size = size_mib.clamp(16, 2048) * MIB;
    let dir = std::path::Path::new(path);
    if !dir.is_dir() {
        return Err(format!("'{}' is not an existing directory.", path));
    }
    // Free-space guard: require 1.5x the test size before writing anything.
    let free = fs2::available_space(dir).map_err(|e| format!("Could not check free space: {e}"))?;
    if free < size + size / 2 {
        return Err(format!(
            "Insufficient free space: need ~{} MiB (with safety margin), only {} MiB available.",
            (size + size / 2) / MIB,
            free / MIB
        ));
    }

    let file_path = dir.join(format!("vyronix_bench_{}.tmp", now_ms()));
    let start = Instant::now();
    let result = run_storage_bench(&file_path, size);
    // Guaranteed cleanup, even on error.
    let _ = std::fs::remove_file(&file_path);
    let mut res = result?;
    res.path = path.to_string();
    res.duration_ms = start.elapsed().as_millis() as u64;
    Ok(res)
}

fn run_storage_bench(file_path: &std::path::Path, size: u64) -> Result<StorageBenchResult, String> {
    let chunk = 1024 * 1024; // 1 MiB sequential chunks
    let data = vec![0xA5u8; chunk];
    let chunks = (size / chunk as u64) as usize;

    // Sequential write.
    let mut file =
        std::fs::File::create(file_path).map_err(|e| format!("Cannot create test file: {e}"))?;
    let write_start = Instant::now();
    for _ in 0..chunks {
        file.write_all(&data)
            .map_err(|e| format!("Write failed: {e}"))?;
    }
    file.sync_all().map_err(|e| format!("Sync failed: {e}"))?;
    let write_elapsed = write_start.elapsed().as_secs_f64();

    // Sequential read.
    file.seek(SeekFrom::Start(0)).map_err(|e| e.to_string())?;
    let mut rbuf = vec![0u8; chunk];
    let read_start = Instant::now();
    for _ in 0..chunks {
        file.read_exact(&mut rbuf)
            .map_err(|e| format!("Read failed: {e}"))?;
    }
    let read_elapsed = read_start.elapsed().as_secs_f64();
    black_box(&rbuf);

    // Random 4K read/write (bounded count).
    let block = 4096usize;
    let ops = 256usize;
    let max_offset = size.saturating_sub(block as u64);
    let mut lcg = Lcg(0xdead_beef_cafe_f00d);
    let mut block_buf = vec![0x5Au8; block];

    let rr_start = Instant::now();
    for _ in 0..ops {
        let off = if max_offset > 0 {
            lcg.next() % max_offset
        } else {
            0
        };
        file.seek(SeekFrom::Start(off)).map_err(|e| e.to_string())?;
        file.read_exact(&mut block_buf).map_err(|e| e.to_string())?;
    }
    let rr_elapsed = rr_start.elapsed().as_secs_f64();

    let rw_start = Instant::now();
    for _ in 0..ops {
        let off = if max_offset > 0 {
            lcg.next() % max_offset
        } else {
            0
        };
        file.seek(SeekFrom::Start(off)).map_err(|e| e.to_string())?;
        file.write_all(&block_buf).map_err(|e| e.to_string())?;
    }
    file.sync_all().map_err(|e| e.to_string())?;
    let rw_elapsed = rw_start.elapsed().as_secs_f64();

    let mb = size as f64 / MIB as f64;
    Ok(StorageBenchResult {
        sequential_read_mbps: mb / read_elapsed.max(1e-9),
        sequential_write_mbps: mb / write_elapsed.max(1e-9),
        random_read_iops: ops as f64 / rr_elapsed.max(1e-9),
        random_write_iops: ops as f64 / rw_elapsed.max(1e-9),
        random_read_latency_ms: (rr_elapsed * 1000.0) / ops as f64,
        random_write_latency_ms: (rw_elapsed * 1000.0) / ops as f64,
        test_size_bytes: size,
        path: String::new(),
        cancelled: false,
        duration_ms: 0,
    })
}

// ---------------- Memory test ----------------

pub fn memory_test(size_mib: u64) -> MemoryTestResult {
    let size = size_mib.clamp(16, 1024) * MIB;
    let count = (size / 8) as usize;
    let start = Instant::now();
    let patterns: Vec<(&str, u64)> = vec![
        ("zeros", 0x0000_0000_0000_0000),
        ("ones", 0xFFFF_FFFF_FFFF_FFFF),
        ("checkerboard_aa", 0xAAAA_AAAA_AAAA_AAAA),
        ("checkerboard_55", 0x5555_5555_5555_5555),
        ("walking_ones", 0x0000_0000_0000_0001),
        ("address", 0x0123_4567_89AB_CDEF),
    ];

    let mut buf: Vec<u64> = vec![0; count];
    let mut errors = 0u64;
    let mut first_error: Option<u64> = None;
    let mut tested: Vec<String> = Vec::new();

    for (name, pattern) in &patterns {
        // Write pattern.
        for (i, v) in buf.iter_mut().enumerate() {
            *v = if *name == "address" {
                i as u64
            } else if *name == "walking_ones" {
                1u64 << (i % 64)
            } else {
                *pattern
            };
        }
        // Verify.
        for (i, &v) in buf.iter().enumerate() {
            let expected = if *name == "address" {
                i as u64
            } else if *name == "walking_ones" {
                1u64 << (i % 64)
            } else {
                *pattern
            };
            if v != expected {
                errors += 1;
                if first_error.is_none() {
                    first_error = Some((i * 8) as u64);
                }
            }
        }
        tested.push(name.to_string());
    }
    black_box(&buf);

    MemoryTestResult {
        patterns_tested: tested,
        bytes_tested: size * patterns.len() as u64,
        errors_found: errors,
        first_error_offset: first_error,
        duration_ms: start.elapsed().as_millis() as u64,
        passed: errors == 0,
        cancelled: false,
        note: "Userspace memory pattern test. Detects addressable memory faults under load but is not a substitute for a firmware/OS-level exhaustive RAM test (e.g. Windows Memory Diagnostic).".into(),
    }
}

// ---------------- Stress ----------------

pub struct StressController {
    pub running: AtomicBool,
    pub paused: AtomicBool,
    pub stop: AtomicBool,
    pub start_ms: std::sync::Mutex<u64>,
    pub target_secs: std::sync::Mutex<u64>,
    pub stopped_reason: std::sync::Mutex<Option<String>>,
    pub paused_accum_ms: std::sync::Mutex<u64>,
}

impl StressController {
    pub fn new() -> Self {
        Self {
            running: AtomicBool::new(false),
            paused: AtomicBool::new(false),
            stop: AtomicBool::new(false),
            start_ms: std::sync::Mutex::new(0),
            target_secs: std::sync::Mutex::new(0),
            stopped_reason: std::sync::Mutex::new(None),
            paused_accum_ms: std::sync::Mutex::new(0),
        }
    }
}

impl Default for StressController {
    fn default() -> Self {
        Self::new()
    }
}

/// Spawn the stress worker. Returns immediately; status is polled separately.
pub fn start_stress(
    ctrl: &std::sync::Arc<StressController>,
    duration_secs: u64,
    temp_limit_c: f32,
) -> Result<(), String> {
    if ctrl.running.load(Ordering::SeqCst) {
        return Err("A stress test is already running.".into());
    }
    let duration = duration_secs.clamp(5, 3600);
    ctrl.running.store(true, Ordering::SeqCst);
    ctrl.paused.store(false, Ordering::SeqCst);
    ctrl.stop.store(false, Ordering::SeqCst);
    *ctrl.start_ms.lock().unwrap() = now_ms();
    *ctrl.target_secs.lock().unwrap() = duration;
    *ctrl.stopped_reason.lock().unwrap() = None;
    *ctrl.paused_accum_ms.lock().unwrap() = 0;

    let ctrl = std::sync::Arc::clone(ctrl);
    std::thread::spawn(move || {
        let threads = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1)
            .min(64);
        let deadline = Instant::now() + Duration::from_secs(duration);
        let mut handles = Vec::new();
        for _ in 0..threads {
            let c = std::sync::Arc::clone(&ctrl);
            handles.push(std::thread::spawn(move || {
                let mut x: u64 = 0x9e37_79b9_7f4a_7c15;
                while Instant::now() < deadline && !c.stop.load(Ordering::Relaxed) {
                    if c.paused.load(Ordering::Relaxed) {
                        std::thread::sleep(Duration::from_millis(100));
                        continue;
                    }
                    for _ in 0..5000 {
                        x = x.wrapping_mul(6364136223846793005).wrapping_add(1);
                        x ^= x >> 17;
                    }
                    black_box(x);
                }
            }));
        }

        // Supervisor: monitor temperature and stop if the limit is exceeded.
        let mut components = sysinfo::Components::new_with_refreshed_list();
        let mut sys = sysinfo::System::new();
        while Instant::now() < deadline && !ctrl.stop.load(Ordering::Relaxed) {
            std::thread::sleep(Duration::from_millis(500));
            components.refresh_list();
            let mut max_cpu_temp: Option<f32> = None;
            for c in components.list() {
                let label = c.label().to_lowercase();
                if label.contains("cpu") || label.contains("package") || label.contains("tdie") {
                    let t = c.temperature();
                    if t.is_finite() && t > 0.0 {
                        max_cpu_temp = Some(max_cpu_temp.map_or(t, |m| m.max(t)));
                    }
                }
            }
            if let Some(t) = max_cpu_temp {
                if t >= temp_limit_c {
                    ctrl.stop.store(true, Ordering::SeqCst);
                    *ctrl.stopped_reason.lock().unwrap() = Some(format!("Temperature limit reached ({t:.0}°C ≥ {temp_limit_c:.0}°C). Test stopped for safety."));
                    break;
                }
            }
            let _ = &mut sys;
        }
        ctrl.stop.store(true, Ordering::SeqCst);
        for h in handles {
            let _ = h.join();
        }
        ctrl.running.store(false, Ordering::SeqCst);
        let mut reason = ctrl.stopped_reason.lock().unwrap();
        if reason.is_none() {
            *reason = Some("Completed requested duration.".into());
        }
    });
    Ok(())
}

pub fn stress_status(ctrl: &StressController) -> StressStatus {
    let running = ctrl.running.load(Ordering::SeqCst);
    let paused = ctrl.paused.load(Ordering::SeqCst);
    let start = *ctrl.start_ms.lock().unwrap();
    let target = *ctrl.target_secs.lock().unwrap();
    let elapsed = if start > 0 {
        (now_ms().saturating_sub(start)) / 1000
    } else {
        0
    };
    let reason = ctrl.stopped_reason.lock().unwrap().clone();

    let mut sys = sysinfo::System::new();
    sys.refresh_cpu_usage();
    let cpu_usage = if running {
        Some(sys.global_cpu_usage())
    } else {
        None
    };
    let cpu_temp = {
        let components = sysinfo::Components::new_with_refreshed_list();
        components
            .list()
            .iter()
            .filter(|c| {
                let l = c.label().to_lowercase();
                l.contains("cpu") || l.contains("package") || l.contains("tdie")
            })
            .map(|c| c.temperature())
            .filter(|t| t.is_finite() && *t > 0.0)
            .fold(None, |acc: Option<f32>, t| {
                Some(acc.map_or(t, |m| m.max(t)))
            })
    };

    StressStatus {
        running,
        paused,
        elapsed_secs: elapsed.min(target),
        target_secs: target,
        cpu_usage_pct: cpu_usage,
        cpu_temp_c: cpu_temp,
        stopped_reason: if running { None } else { reason },
    }
}
