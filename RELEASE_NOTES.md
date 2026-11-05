# VYRONIX v0.1.0 — Pre-release

**Know Your Hardware. Trust Your Device.** · A MERQO Software Product

> **Status: pre-release.** This is the first public build of VYRONIX. It is functional
> and compiled/packaged for Windows 10/11 x64, but **on-hardware validation across a
> range of physical devices is ongoing**. Treat results as informative, and verify
> critical decisions independently.

## What's included

A premium, privacy-first Windows PC & laptop diagnostic ecosystem:

- **Full hardware inventory** — system, motherboard, BIOS, OS, CPU, GPU, memory
  (per-DIMM), battery, storage, network adapters.
- **Battery health** — design/full-charge capacity, validated health & wear, cycles
  (where firmware reports them), charge/discharge rate, runtime estimate.
- **Storage SMART** — drive identity, health, wear, temperature, power-on hours and
  error counters from the Windows Storage interface (no admin required).
- **Real-time monitoring** — CPU, memory, network throughput, temperatures where exposed.
- **Interactive tests** — display patterns, keyboard, mouse/touchpad, speaker & microphone.
- **Benchmarks** — CPU (single/multi), memory bandwidth/latency, storage sequential &
  random I/O (safe temp-file method).
- **Stress test** — bounded CPU load with duration + temperature safety limits.
- **Windows & network diagnostics** — processes, services, drivers, startup, critical
  events; ICMP ping and DNS.
- **Guided inspection** — 29-category checklist.
- **Transparent diagnostic engine** — rule-based findings + weighted health score that
  excludes unsupported items (never counts Unknown as Good).
- **Branded reports** — PDF / HTML / CSV / JSON, fully offline.
- **Premium UI** — dark/light themes, accessibility, global search, settings.

## Downloads

| File | Description |
| --- | --- |
| `VYRONIX-0.1.0-x64-portable.exe` | Portable executable — run directly, no install. |
| `VYRONIX-0.1.0-x64-setup.exe` | NSIS installer (per-machine). |
| `SHA256SUMS.txt` | SHA-256 checksums for integrity verification. |

**Verify checksums** before running:
```powershell
Get-FileHash .\VYRONIX-0.1.0-x64-portable.exe -Algorithm SHA256
```

## Important notes

- **Unsigned build.** No code-signing certificate is used. Windows SmartScreen may show
  a warning on first run — this is expected for unsigned software. Verify the SHA-256
  checksum, and choose "More info → Run anyway" only if you trust the source.
- **Honest data.** VYRONIX never fabricates values. Anything it cannot measure is shown
  as `Unsupported` / `Unavailable` / `Permission Required` with an explanation. Some
  sensors (CPU/GPU temperature, battery cycles, detailed ATA SMART) depend on what your
  hardware/firmware exposes and may be unavailable.
- **Privacy.** No telemetry, no accounts, no cloud. Everything runs locally; the only
  network calls are user-initiated ping/DNS.
- **Safety.** Read-only by default. Storage benchmarks use dedicated temp files with a
  free-space guard and cleanup. No registry/driver/security changes, no reboot.

## Known limitations

See [docs/LIMITATIONS.md](docs/LIMITATIONS.md). Notably: GPU utilization/temperature and
some SMART details require interfaces not bundled here; the memory test is a userspace
pattern test, not a firmware-level exhaustive RAM test.

## Validation status

- Core logic: **72 unit tests passing**; clippy clean.
- Frontend: **24 unit tests passing**; typecheck + production build clean.
- Backend: **compiles for Windows x64** (`CHECK_EXIT=0`); launch smoke test on CI.
- **On-hardware readings have not yet been validated across physical devices** — this is
  the primary focus for the next iteration.

## Credits

Owner / Founder: **Shohan Khan** · Contact: **merqoonline@gmail.com** · Brand: **MERQO**
© Shohan Khan. All rights reserved. VYRONIX is an original product mark.
