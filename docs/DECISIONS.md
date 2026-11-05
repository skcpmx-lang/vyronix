# VYRONIX Decision Log

Routine engineering/technology decisions are made independently and recorded here.
Only choices that materially affect the product are listed.

| # | Decision | Rationale | Alternatives considered |
| --- | --- | --- | --- |
| D1 | **Tauri 2** native shell (Rust backend + web frontend) | Small binary, native Windows access, no bundled browser runtime bloat; matches "real desktop app" goal. | Electron (heavier), pure Win32/WPF (slower UI iteration), .NET MAUI. |
| D2 | **React 18 + TS + Vite + Tailwind** frontend | Fast iteration, strong typing, premium UI, offline bundle. | Svelte, Vue, vanilla. |
| D3 | **Pure core crate + OS-specific backend** separation | Core logic is unit-testable on any OS; hardware code isolated and `cfg(windows)`. | Monolith mixing logic + OS calls (untestable in sandbox). |
| D4 | **`sysinfo` 0.32.1** pinned | Verified API; edition 2021 / MSRV fits CI. 0.40 requires newer MSRV/edition. | sysinfo 0.40 (rejected: MSRV), raw Win32 for everything. |
| D5 | **`starship-battery` 0.11.1** for battery | Maintenance fork of rust-battery used by Starship; verified API; Windows battery IOCTL. | rust-battery 0.7.8 (older uom), WMI-only (less accurate capacity). |
| D6 | **WMI via `wmi` 0.18.4**, results → JSON | Read-only CIM queries; defensive `Variant→JSON` avoids type-mismatch panics. | Typed structs (brittle to WMI type variance), PowerShell-only (slow). |
| D7 | **SMART via `MSFT_PhysicalDisk`/`MSFT_StorageReliabilityCounter`** (no admin) | Genuine SMART-derived counters on Win8+ without elevation. Admin-only ATA/NVMe detail honestly `Unsupported`. | Raw IOCTL/`root\WMI` (needs admin), vendor tools (unbundleable). |
| D8 | **No LibreHardwareMonitor driver** | Bundling a kernel driver raises security/AV/signing concerns; temps only from exposed sensors. | LHM driver (rejected: safety/signing). |
| D9 | **ICMP via `IcmpSendEcho`** (IP Helper) | No-admin ping with real RTT/loss. | Raw sockets (admin), parsing `ping.exe` (locale-fragile). |
| D10 | **SQLite via `rusqlite` (bundled)** | Local, dependency-free persistence; no external DB. | JSON files (weaker querying), sled. |
| D11 | **PDF client-side (jsPDF + autotable)** | Fully offline branded PDF from the same report model; no server. | Server-side PDF (needs a service), printpdf (API unverifiable offline). |
| D12 | **Score = weighted over included components only** | Unknown/unsupported must not inflate or deflate health; confidence from coverage. | Universal score counting unknowns as good (dishonest). |
| D13 | **CPU bench score = Mops/s** | Reproducible, clearly-labelled physical metric, not an arbitrary index. | Opaque "points". |
| D14 | **Unsigned builds** | No paid code-signing certificate per constraint; documented SmartScreen expectation. | Paid cert (out of scope). |
| D15 | **CI diagnostics commit-back** | Sandbox has no Rust toolchain and CI log host is blocked; exact compiler output is committed to `.ci-*diagnostics.txt`. Removed before release. | Downloading logs (blocked). |
| D16 | **Original VYRONIX icon generated with Pillow** | Distinct product mark; no MERQO logo imitation; no invented copyright year/trademark claim. | Using MERQO corporate logo (prohibited). |

## Reversed / superseded

- Initially assumed a `smart::SmartData/evaluate` API; corrected to the real core
  SMART surface and a conservative MSFT-counter evaluation in the backend.
- Battery version moved 0.12.0 → **0.11.1** after confirming the version Starship
  actually pins (the fork's source repo is no longer publicly reachable).
