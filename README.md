# VYRONIX

**Know Your Hardware. Trust Your Device.**

VYRONIX is a premium, privacy-first **Windows 10/11 x64 PC & laptop diagnostic,
hardware-health monitoring, testing and inspection** desktop ecosystem.
A **MERQO** software product.

- **Owner / Founder:** Shohan Khan
- **Contact:** merqoonline@gmail.com
- **Brand:** MERQO · **Product:** VYRONIX
- **License:** Proprietary — © Shohan Khan. All rights reserved.

> VYRONIX reports only what it can genuinely measure. Every value carries an
> honest status (`Available`, `Verified`, `Estimated`, `Unsupported`,
> `Permission Required`, `Unavailable`, `Failed`, `Not Tested`,
> `Manual Verification Required`), a source and a confidence level. **Unknown is
> never shown as Good, and Estimated is never shown as measured.** No telemetry,
> no trackers, no subscriptions, no paid APIs — everything runs locally.

---

## What it does

| Area | Capability |
| --- | --- |
| **System inventory** | Device, motherboard, BIOS, OS, Secure Boot, TPM, virtualization |
| **CPU / GPU / Memory** | Identification, clocks, usage, DIMM topology; temperatures only from real sensors |
| **Battery** | Design/full-charge capacity, validated health & wear, cycles, charge/discharge rate, runtime estimate |
| **Storage** | Drive identity, SMART-derived health, wear, temperature, power-on hours, partitions |
| **Real-time monitoring** | CPU, memory, network throughput, temperatures (where exposed) |
| **Interactive tests** | Display patterns, keyboard, mouse/touchpad, speaker & microphone |
| **Benchmarks** | CPU (single/multi), memory bandwidth/latency, storage sequential & random I/O |
| **Stress test** | Bounded CPU load with duration + temperature safety limits, pause/stop |
| **Windows & network** | Processes, services, drivers, startup, critical events; adapters, ICMP ping, DNS |
| **Guided inspection** | 29-category physical/functional checklist with honest status tracking |
| **Diagnostic engine** | Transparent rule-based findings + weighted health score (unsupported items excluded) |
| **Reports** | Branded **PDF / HTML / CSV / JSON**, fully offline |
| **Premium UI** | Dark/light themes, accessibility, global search, settings, animations |

## Architecture

```
vyronix/
├─ crates/vyronix-core/        # Pure, unit-tested Rust logic (no OS calls):
│                              #   models, battery math, SMART interpretation,
│                              #   scoring, diagnostics, recommendations, reports
├─ apps/desktop/
│  ├─ src/                     # React 18 + TypeScript + Vite + Tailwind UI (21 pages)
│  └─ src-tauri/               # Tauri 2 native backend (Windows hardware collection,
│                              #   benchmarks, SQLite, logging, IPC commands)
├─ assets/                     # Original VYRONIX branding & icons
├─ docs/                       # Architecture, plan, status, validation, release gates
└─ .github/workflows/          # CI (core + frontend), Windows backend check, release
```

The **core crate is platform-independent and fully unit-tested**. The **desktop
backend** collects real hardware data on Windows via WMI/CIM, `sysinfo`, the
Windows battery interface and the Windows Storage management namespace, and
falls back to honest "unavailable" states elsewhere. The **frontend** renders
measurements with provenance and never fabricates data; in a plain browser it
shows a clear "desktop runtime required" notice instead of mock hardware.

## Building

> The native target is **Windows 10/11 x64**. The Rust backend is compiled and
> packaged on Windows (see CI). A browser dev-preview of the UI is possible but
> cannot read hardware.

```bash
# Frontend (UI) — from apps/desktop
npm ci
npm run dev        # browser preview (no hardware access)
npm run test       # vitest unit tests
npm run build      # production bundle

# Core logic — from repo root
cargo test -p vyronix-core

# Native Windows app — from apps/desktop (on Windows, Rust + Node installed)
npm run tauri build   # produces installer + portable executable
```

## Honesty & safety guarantees

- No fabricated specs, sensor readings, health %, benchmark scores or cycle counts.
- Storage benchmarks use **dedicated temp files only**, with a free-space guard,
  cancellation and guaranteed cleanup — never raw/destructive device writes.
- No automatic reboot, driver install/uninstall, registry modification, or
  disabling of Windows security features. Least-privilege, read-only by default.
- No secret audio recording or typed-content capture; the microphone test shows a
  live level meter only, and the keyboard test never logs what you type.
- Builds are **unsigned** (no paid code-signing certificate).

## Project status

See **[docs/PROJECT_STATUS.md](docs/PROJECT_STATUS.md)** for the live feature/status
matrix and **[docs/RELEASE_READINESS.md](docs/RELEASE_READINESS.md)** for the release
gates and physical-hardware validation limits. VYRONIX is under active development;
a release is published only after every release gate passes.

---

*VYRONIX is an original product mark. It does not reproduce or imitate the MERQO
corporate logo, and no trademark registration is claimed.*
