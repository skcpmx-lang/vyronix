# VYRONIX Release Readiness & Gates

**Current state:** a **draft pre-release `v0.1.0`** exists with the portable exe, NSIS
installer and SHA-256 checksums. All **automatable** gates below pass. **Publication is
deliberately withheld** until the owner explicitly authorizes it. Physical-device testing
is performed **manually by the owner** and does **not** block preparing the draft release.

Nothing is described as "released", "shipped", or "works on hardware" without evidence.

## Release gates

| # | Gate | Status | Evidence |
| --- | --- | --- | --- |
| G1 | Core crate tests pass | ✅ | `ci.yml`: 72 tests, 0 failed. |
| G2 | Core clippy clean (`-D warnings`) | ✅ | `ci.yml`. |
| G3 | Frontend typecheck + unit tests + build pass | ✅ | `ci.yml` frontend job: 24 tests, build OK. |
| G4 | Tauri backend compiles for `x86_64-pc-windows-msvc` | ✅ | `backend.yml`: `cargo check --workspace` clean. |
| G5 | Windows packaging produces **portable exe + NSIS installer** | ✅ | portable 7,134,720 B + setup ~2.69 MB. |
| G6 | **Launch smoke test** — exe starts, stays running, no crash | ✅ | `package.yml` + `release.yml` smoke step passed. |
| G7 | Checksums (SHA-256) generated for artifacts | ✅ | `SHA256SUMS.txt` (release asset). |
| G8 | Release notes drafted (honest, versioned) | ✅ | `RELEASE_NOTES.md`. |
| G9 | No secrets / build artifacts committed; `.gitignore` correct | ✅ | repo review. |
| G10 | Temporary CI diagnostic helpers removed; `cargo fmt --check` restored | ✅ | `format.yml` + `.ci-*` deleted; fmt check in `ci.yml`. |
| G11 | Physical-device testing **process** documented; results owner-recorded | ✅ (process) | `MANUAL_HARDWARE_TESTING.md` + report template + issue forms. Results pending owner. |
| G12 | Version + tag consistent (`VERSION`, Cargo, tauri.conf, package.json) | ✅ | all 0.1.0. |
| G13 | **Owner authorization to publish** | ⏳ **required** | Explicit owner approval; release stays a draft until then. |

## Validation levels & physical testing

- **Automated (Level A)** and **Windows CI (Level B)** validation are complete and prove the
  app builds, packages and launches. See `VALIDATION.md`.
- **Physical-device validation (Level C)** is performed **manually by the owner** on real
  Windows 10/11 computers using `MANUAL_HARDWARE_TESTING.md`. It is **not a blocker** for
  preparing the draft release, and its results are recorded in `VALIDATION.md` **only after**
  the owner personally runs the tests and submits them.
- Until Level C results exist, no component is claimed as hardware-validated, and the
  documented limitations/unsupported states remain unchanged and honest.

## Physical-hardware validation limits (honest disclosure)

The product is developed in a **Linux sandbox with no Windows device and no local Rust
toolchain**. Therefore:

- Rust correctness is proven by **CI compilation + unit tests**, not by running on the
  developer's Windows machine.
- Hardware collection is implemented against **documented, verified Windows interfaces**
  (WMI/CIM, sysinfo, battery IOCTL, Storage namespace, IP Helper), but **actual readings on a
  specific physical PC/laptop have not been observed** by the developer.
- Interactive tests (display/keyboard/mouse/audio) are implemented and unit/build verified,
  but **require a human at a real device** to confirm behaviour.
- The launch smoke test on Windows CI proves the app **starts and renders**; it does not prove
  every sensor on every machine.

**Consequence:** the release is a **pre-release (v0.1.0)** with these limits stated in the
release notes. Claims like "validated on real hardware" are made only after the owner performs
and records a physical-device test.

## What makes a feature "Validated"

- Logic features: passing automated tests (core/frontend).
- Collection features: a recorded run on Windows showing real values with correct
  status/source, OR a documented interface-level verification + CI smoke.
- Interactive features: a recorded manual test on a real device (owner).
- Packaging: CI-produced artifacts + a successful launch smoke test + checksums.

## Release procedure

1. ✅ Align versions; remove diagnostic helpers; restore `cargo fmt --check`.
2. ✅ Build portable exe + NSIS installer on Windows CI; compute SHA-256; smoke test.
3. ✅ Write honest `RELEASE_NOTES.md`.
4. ✅ Create tag `v0.1.0`; create a **draft pre-release** with artifacts + checksums + notes.
5. ⏳ **Owner performs manual hardware testing** (`MANUAL_HARDWARE_TESTING.md`) and submits
   results; record them in `VALIDATION.md`.
6. ⏳ **Owner authorizes publication** → only then publish the pre-release. Do **not** delete
   release assets or rewrite release history.
7. Update `PROJECT_STATUS.md` and `CHANGELOG.md` with the publication evidence.
