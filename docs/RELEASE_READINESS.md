# VYRONIX Release Readiness & Gates

**A GitHub Release is published only when EVERY gate below passes.** Until then there
is no release, and nothing is described as "released", "shipped", or "works on
hardware" without evidence. Current state: **pre-release, gates NOT all satisfied.**

## Release gates

| # | Gate | Status | Evidence required |
| --- | --- | --- | --- |
| G1 | Core crate tests pass | ✅ | CI `TEST_EXIT=0`, 72 tests. |
| G2 | Core clippy clean (`-D warnings`) | ✅ | CI `CLIPPY_EXIT=0`. |
| G3 | Frontend typecheck + unit tests + build pass | ✅ (local) / 🔄 (CI job to add) | vitest 24 pass; Vite build OK; add CI job. |
| G4 | Tauri backend compiles for `x86_64-pc-windows-msvc` | 🔄 | `.ci-backend-diagnostics.txt` `CHECK_EXIT=0`. |
| G5 | Windows packaging produces **portable exe + NSIS installer** | ⏳ | CI build artifacts exist. |
| G6 | **Launch smoke test** — exe starts, window renders, no crash | ⏳ | CI smoke step / recorded run. |
| G7 | Checksums (SHA-256) generated for artifacts | ⏳ | `SHA256SUMS.txt`. |
| G8 | Release notes drafted (honest, versioned) | ⏳ | `RELEASE_NOTES.md`. |
| G9 | No secrets / build artifacts committed; `.gitignore` correct | ✅ | repo review. |
| G10 | Temporary CI diagnostic helpers removed; `cargo fmt --check` restored | ⏳ | delete `format.yml`/`backend.yml` diagnostics + `.ci-*` files before release. |
| G11 | On-hardware validation documented (what was/wasn't tested) | ⏳ | this file + `VALIDATION.md`. |
| G12 | Version + tag consistent (`VERSION`, Cargo, tauri.conf, package.json) | 🔄 | align to 0.1.0. |

## Physical-hardware validation limits (honest disclosure)

The product is developed in a **Linux sandbox with no Windows device and no local
Rust toolchain**. Therefore:

- Rust correctness is proven by **CI compilation + unit tests**, not by running on
  the developer's Windows machine.
- Hardware collection is implemented against **documented, verified Windows
  interfaces** (WMI/CIM, sysinfo, battery IOCTL, Storage namespace, IP Helper), but
  **actual readings on a specific physical PC/laptop have not been observed** here.
- Interactive tests (display/keyboard/mouse/audio) are implemented and unit/build
  verified, but **require a human at a real device** to confirm behaviour.
- The launch smoke test on Windows CI proves the app **starts and renders**; it does
  not prove every sensor on every machine.

**Consequence:** the first release will be labelled a **pre-release (v0.1.0)** with
these limits stated in the release notes. Claims like "validated on real hardware"
will only be made after a physical-device test is actually performed and recorded.

## What would make a feature "Validated"

- Logic features: passing automated tests (core/frontend).
- Collection features: a recorded run on Windows showing real values with correct
  status/source, OR a documented interface-level verification + CI smoke.
- Interactive features: a recorded manual test on a real device.
- Packaging: CI-produced artifacts + a successful launch smoke test + checksums.

## Release procedure (once gates pass)

1. Align versions; remove diagnostic helpers; restore `cargo fmt --check`.
2. Build portable exe + NSIS installer on Windows CI; compute SHA-256.
3. Write honest `RELEASE_NOTES.md` (features, limits, unsigned/SmartScreen note).
4. Create tag `v0.1.0`; publish a **pre-release** with artifacts + checksums + notes.
5. Update `PROJECT_STATUS.md` and `CHANGELOG.md` with the release evidence.
