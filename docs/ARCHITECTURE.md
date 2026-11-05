# VYRONIX Architecture

## Goals & principles

1. **Real data only.** Every hardware value is collected from a genuine OS/hardware
   interface and carries `status`, `source`, `confidence`, and an optional `note`.
   Nothing is simulated in production.
2. **Honest classification.** `Unknown ≠ Good`, `Estimated ≠ measured`. Unsupported
   items are excluded from scoring denominators, never counted as healthy.
3. **Privacy & offline-first.** No telemetry, no network calls except user-initiated
   ping/DNS. All persistence is local SQLite; logs stay on-device.
4. **Least privilege & safety.** Read-only by default. No destructive storage writes,
   no registry/driver/security changes, no reboot. Admin is only *requested* where a
   data source genuinely needs it, and the UI shows `Permission Required` otherwise.
5. **Separation of concerns.** Pure logic lives in a platform-independent, unit-tested
   core crate; OS-specific collection lives in the desktop backend; presentation lives
   in the frontend.

## Workspace layout

| Crate / package | Responsibility |
| --- | --- |
| `crates/vyronix-core` | Pure Rust: data model, battery health math, SMART interpretation (ATA/NVMe), weighted scoring, rule-based diagnostics, use-case recommendations, report model + JSON/HTML/CSV serialization. **No OS calls.** Fully unit-tested. |
| `apps/desktop/src-tauri` | Tauri 2 native backend: Windows hardware collection, benchmarks/stress, SQLite, logging, IPC commands, report assembly. |
| `apps/desktop` (frontend) | React 18 + TypeScript + Vite + Tailwind. 21 pages, shared components, zustand store, offline PDF generation (jsPDF + autotable). |

## Data model (the honesty contract)

`Measurement<T>` (core) is the universal wrapper:

```rust
Measurement<T> { value: Option<T>, status: Status, confidence: Confidence,
                 source: String, note: Option<String>, timestamp_ms: u64 }
```

`Status` ∈ { Available, Verified, Estimated, Unsupported, PermissionRequired,
Unavailable, Failed, NotTested, ManualVerificationRequired }. The frontend mirrors
these types exactly (`src/lib/types.ts`) and renders status + source + confidence
for every value (`MeasurementValue`, `StatusBadge`).

## Backend collection sources (Windows)

| Domain | Primary source | Notes |
| --- | --- | --- |
| System / motherboard / BIOS / OS | WMI `Win32_ComputerSystem`, `Win32_BaseBoard`, `Win32_BIOS`, `Win32_OperatingSystem` + `sysinfo` | Secure Boot via read-only registry query; TPM via `Win32_Tpm` (may need admin → `Permission Required`). |
| CPU | `sysinfo` + WMI `Win32_Processor` | Temperature only from a labelled `sysinfo` sensor; never inferred from usage. |
| GPU | WMI `Win32_VideoController` | Utilization/temperature marked `Unsupported` (no generic interface bundled). |
| Memory | `sysinfo` + WMI `Win32_PhysicalMemory` | Per-DIMM capacity/speed/type/slot. |
| Battery | `starship-battery` (IOCTL) + WMI `Win32_Battery` | Health/wear from validated core math on design/full-charge capacity. |
| Storage | WMI `Win32_DiskDrive` + `MSFT_PhysicalDisk` / `MSFT_StorageReliabilityCounter` (root\Microsoft\Windows\Storage) | Real SMART-derived counters without admin. Detailed ATA/NVMe attributes needing the admin pass-through are `Unsupported`. |
| Network | WMI `Win32_NetworkAdapter`/`Configuration`; ICMP via `IcmpSendEcho`; DNS via `getaddrinfo` | Wi-Fi signal `Unsupported` (needs WLAN API). |
| Monitoring | persistent `sysinfo` (`System`, `Networks`, `Components`) | Rates computed across refresh intervals. |
| Windows diagnostics | `sysinfo` processes; WMI services/drivers/startup; `Get-WinEvent` (read-only) for critical/error events | Row caps to bound cost. |

WMI results are converted `Variant → serde_json::Value` defensively; every query
failure becomes an honest error/unavailable state, never fabricated data.

## Benchmarks & stress

- **CPU:** deterministic mixed integer/float loop, single- and multi-core; score is
  **millions of operations/second** (clearly labelled, reproducible).
- **Memory:** sequential write/read/copy bandwidth + random-access latency.
- **Storage:** dedicated temp file; free-space guard (1.5×), sequential + random 4K
  I/O, guaranteed cleanup. Never touches raw devices.
- **Memory test:** userspace pattern write/verify (zeros/ones/checkerboards/walking
  ones/address). Documented as *not* a firmware-level exhaustive RAM test.
- **Stress:** bounded worker threads; supervisor stops on temperature limit or
  duration; pause/stop via atomics; status polled.

## Diagnostic engine & scoring

`core::diagnostics::evaluate` produces transparent findings from collected facts.
`core::scoring::compute_overall` builds a weighted 0–100 score over **included**
components only (Good=100, Information=80, Warning=50, Critical=15; Unknown excluded).
Confidence derives from coverage (High ≥75%, Medium ≥50%, Low below, Insufficient if
none). `core::recommend::recommend` maps hardware facts to use-case suitability.

## Reports

`core::report::ReportModel` is assembled from real data, then serialized to
**JSON / HTML (escaped, branded) / CSV**. The frontend builds **PDF** offline from the
same model (jsPDF + autotable). `build_report` returns `{ model, html, csv, json }`;
the UI picks the format. Sensitive identifiers are included only with explicit consent.

## Frontend

React 18 + TS + Vite + Tailwind; zustand store; lucide icons; custom SVG charts/gauges.
21 pages routed by a module registry with a global search index. In a browser (no
`window.__TAURI_INTERNALS__`) every service returns an honest unavailable state and the
UI shows a "desktop runtime required" notice — **no mock hardware**.

## IPC contract

~27 Tauri commands (camelCase args from the frontend, snake_case in Rust). See
`apps/desktop/src/lib/services.ts` and `apps/desktop/src-tauri/src/commands.rs`.
Plugins are limited to `dialog` (save picker) and `opener`; hardware access is via
validated custom commands, not broad plugin permissions.
