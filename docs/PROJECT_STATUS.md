# VYRONIX Project Status

_Last updated: 2026-11-05 · version 0.1.0 (pre-release, in development)_

This is the live tracking document. It distinguishes **Implemented** (code exists)
from **Validated** (evidence proves it works). Nothing is marked complete/validated
without evidence. Hardware-dependent behaviour can only be fully validated on real
Windows 10/11 x64 hardware; that limit is recorded in `RELEASE_READINESS.md`.

## Verification snapshot

| Check | Result | Evidence |
| --- | --- | --- |
| Core crate unit tests | ✅ **72 passed, 0 failed** | CI `.ci-diagnostics.txt` (`TEST_EXIT=0`) |
| Core clippy (`-D warnings`) | ✅ **clean** | CI `.ci-diagnostics.txt` (`CLIPPY_EXIT=0`) |
| Frontend typecheck (`tsc --noEmit`) | ✅ clean | local run |
| Frontend unit tests (vitest) | ✅ **24 passed** | local run |
| Frontend production build (Vite) | ✅ succeeds | local run + Windows CI |
| Original icon set (incl. `.ico`) | ✅ generated | `scripts/generate_icons.py` |
| Tauri backend Windows compile | ✅ **`CHECK_EXIT=0`** | `.ci-backend-diagnostics.txt` (Windows CI) |
| Native app on-hardware validation | ⏳ pending | requires Windows device |
| Packaging (installer + portable) | ✅ **built + smoke-tested** | portable 6.8 MB + NSIS 2.6 MB; SHA-256 recorded |
| GitHub Release | ✅ **draft pre-release `v0.1.0` created** | portable exe + installer + SHA256SUMS attached; publication gated on authorization |

## Feature status matrix

Legend: ✅ implemented + logic validated · 🖥 implemented, needs on-hardware validation · ⏳ pending.

| Feature | UI | Backend | Notes |
| --- | --- | --- | --- |
| Welcome / Dashboard / About | ✅ | 🖥 | Dashboard consumes diagnostics engine. |
| System overview | ✅ | 🖥 | WMI + sysinfo; Secure Boot/TPM honest states. |
| CPU info | ✅ | 🖥 | Temp only from real sensors. |
| GPU info | ✅ | 🖥 | Utilization/temp `Unsupported` (no generic iface). |
| Memory info | ✅ | 🖥 | Per-DIMM via WMI. |
| Battery diagnostics | ✅ | 🖥 | Validated health math; cycles `Unsupported` if firmware silent. |
| Storage + SMART | ✅ | 🖥 | MSFT reliability counters; ATA detail needs admin. |
| Real-time monitoring | ✅ | 🖥 | Persistent sysinfo; GPU/disk throughput honestly absent. |
| Display test | ✅ | n/a | 15 patterns, fullscreen; visual-inspection disclaimer. |
| Keyboard test | ✅ | n/a | Physical-key map; no content logging. |
| Mouse / touchpad test | ✅ | n/a | Buttons/wheel/move counters; gesture limit noted. |
| Audio / mic test | ✅ | n/a | Web Audio tones; mic live RMS meter only. |
| CPU / memory / storage benchmarks | ✅ | 🖥 | Real workloads; storage temp-file safe. |
| Memory test | ✅ | 🖥 | Userspace patterns; not firmware-level. |
| Stress test | ✅ | 🖥 | Duration + temp limit, pause/stop. |
| Windows diagnostics | ✅ | 🖥 | Read-only; event log via Get-WinEvent. |
| Network (adapters/ping/DNS) | ✅ | 🖥 | ICMP via IP Helper; Wi-Fi signal `Unsupported`. |
| Guided inspection (29) | ✅ | 🖥 | Persisted checklist. |
| Diagnostic engine + scoring | ✅ | ✅ | Core logic unit-tested. |
| Reports PDF/HTML/CSV/JSON | ✅ | 🖥 | Offline PDF; backend bundle. |
| Settings / themes / search | ✅ | 🖥 | Persisted settings. |

## Completed milestones

1. ✅ **Backend Windows compile** — `CHECK_EXIT=0`, 0 warnings (Windows CI).
2. ✅ **Frontend CI job** — typecheck/test/build in `ci.yml`.
3. ✅ **Packaging** — NSIS installer + portable exe; SHA-256 checksums; release notes.
4. ✅ **Launch smoke test** — portable exe launched and stayed running on Windows CI.
5. ✅ **Release gates** — all automatable gates in `RELEASE_READINESS.md` satisfied.
6. ✅ **Cleanup** — temporary diagnostic helpers removed; `cargo fmt --check` restored.
7. ✅ **Draft pre-release `v0.1.0` created** with portable exe + installer + checksums.

## Remaining work

1. **On-hardware validation** — run VYRONIX on real Windows 10/11 devices and record
   actual sensor/benchmark behaviour (the primary limitation of sandbox development).
2. **Publish the release** — the draft pre-release is ready; publication is a deliberate
   step requiring owner authorization (and ideally after on-hardware validation).
3. **Post-release hardening** — strict CSP, dependency auditing, optional code-signing.

## Honesty note

The development environment is a Linux sandbox without a Rust toolchain or Windows
hardware. Rust is compiled via GitHub Actions; hardware behaviour is implemented
against verified OS interfaces but must be validated on a real Windows device before
any claim of "works on hardware" is made. This document will be updated as evidence
arrives.
