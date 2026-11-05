# VYRONIX Requirements Traceability

Maps the requested scope to implementation and validation status.
Status: ✅ implemented + logic validated · 🖥 implemented, on-hardware validation pending · ⏳ pending.

## Functional scope

| Requirement | Implementation | Status |
| --- | --- | --- |
| System / CPU / GPU / motherboard / RAM info | `hw/system.rs`, WMI + sysinfo; pages System/Cpu/Gpu/Memory | 🖥 |
| Battery diagnostics (health, wear, cycles, rate, runtime) | `hw/battery.rs` + core battery math; BatteryPage | 🖥 |
| Storage SMART + benchmarks | `hw/storage.rs` (MSFT counters) + `bench.rs`; StoragePage | 🖥 |
| Real-time monitoring | `hw/monitoring.rs`; MonitoringPage + LiveChart | 🖥 |
| Display test | DisplayTest (15 patterns, fullscreen) | 🖥 |
| Keyboard test | KeyboardTest (physical map, no content capture) | 🖥 |
| Mouse / touchpad test | MouseTest | 🖥 |
| Audio / microphone test | AudioTest (tones + live RMS meter) | 🖥 |
| CPU / GPU / memory / storage benchmarks | `bench.rs`; Benchmarks page | 🖥 |
| Stress tests (bounded, safe) | `bench.rs` StressController; StressTest page | 🖥 |
| Windows diagnostics | `hw/windows.rs`; WindowsDiagnostics page | 🖥 |
| Network diagnostics (adapters/ping/DNS) | `hw/network.rs`; NetworkPage | 🖥 |
| Guided inspection (29 categories) | Inspection page + SQLite persistence | 🖥 |
| Transparent rule-based diagnostic engine | core `diagnostics.rs` | ✅ |
| Weighted scoring (exclude unsupported) | core `scoring.rs` | ✅ |
| Reports PDF / HTML / CSV / JSON | core `report.rs` + frontend `pdf.ts`; Reports page | 🖥 |
| Premium UI (dark/light, a11y, animations, search, settings) | App shell + components + SettingsPage | ✅ (build/tests) |

## Cross-cutting acceptance criteria

| Criterion | How it is met | Status |
| --- | --- | --- |
| Real functionality only, no fabricated data | `Measurement<T>` provenance; honest unavailable states; browser shows "desktop required" | ✅ |
| Honest status classification | 9-value `Status` enum rendered everywhere | ✅ |
| Unknown ≠ Good; Estimated ≠ measured | scoring excludes Unknown; `estimated()` constructor distinct | ✅ |
| No CPU temp inferred from usage | temp only from labelled sensors | ✅ |
| Storage benchmark safety | temp file + free-space guard + cleanup; no raw writes | ✅ |
| Privacy / offline / no telemetry | local SQLite + logs; only user-initiated ping/DNS | ✅ |
| No risky system changes | read-only; no reboot/driver/registry/security changes | ✅ |
| No MERQO logo imitation / no invented year/trademark | original generated icon; honest copyright line | ✅ |
| Unsigned builds | no signing cert; SmartScreen note in docs | ✅ |
| Six tracking docs + architecture/plan | `docs/` suite | ✅ |
| Resumable via CONTINUE | recovery procedure in IMPLEMENTATION_PLAN | ✅ |
| CI/CD | ci.yml (core+frontend), backend.yml (Windows), package.yml, release.yml | ✅ |
| Verified Windows executable + release | packaging + smoke + gates; draft pre-release v0.1.0 | ✅ (draft; publication gated) |
| Manual hardware testing process | `MANUAL_HARDWARE_TESTING.md` + report template + issue forms | ✅ (process); results owner-recorded |
| Physical-device validation | Owner performs on real Windows 10/11; recorded in VALIDATION.md after submission | ⏳ (owner) |

## Non-goals / explicitly out of scope

- Paid code-signing certificate; kernel drivers (e.g. LibreHardwareMonitor);
  vendor-specific GPU tooling; firmware-level exhaustive RAM testing; any telemetry.
