# VYRONIX Validation Record

This file records **actual evidence** of what has been validated. It is updated only as
real evidence arrives. No entry is added without a real, observed result.

## Validation levels (clearly distinguished)

| Level | What it proves | Status |
| --- | --- | --- |
| **A — Automated** | Logic correctness: unit tests, clippy, `rustfmt`, typecheck, production build. | ✅ Done (CI + sandbox) |
| **B — Windows CI** | The app compiles for Windows x64, packages (NSIS + portable), and the exe **launches and stays running** (smoke test); checksums produced. | ✅ Done (`windows-latest` runners) |
| **C — Physical device** | Real sensor readings, interactive tests, benchmarks/stress and peripheral behaviour on actual Windows 10/11 hardware. | ⏳ **Performed manually by the owner**; recorded here only after submission. |

Levels A and B do **not** establish per-device hardware compatibility. **Level C** does, and
it is recorded only from results the owner personally observes and submits (see
[`MANUAL_HARDWARE_TESTING.md`](MANUAL_HARDWARE_TESTING.md)).

## Level A — Automated (observed)

| Item | Result | Where observed |
| --- | --- | --- |
| Core unit tests | **72 passed, 0 failed** | GitHub Actions `ci.yml` |
| Core clippy `-D warnings` | **0 warnings / errors** | `ci.yml` |
| Core `rustfmt --check` | **clean** | `ci.yml` |
| Frontend `tsc --noEmit` | **0 errors** | `ci.yml` frontend job + sandbox |
| Frontend vitest | **24 passed (4 files)** | `ci.yml` frontend job + sandbox |
| Frontend Vite production build | **success** | `ci.yml` frontend job + sandbox |

## Level B — Windows CI (observed)

| Item | Result | Where observed |
| --- | --- | --- |
| Backend Windows compile | ✅ `cargo check --workspace` clean, 0 warnings | `backend.yml` (`windows-latest`) |
| Windows packaging (release + NSIS) | ✅ portable exe + installer built | `package.yml` run 41982545548 |
| Launch smoke test (portable exe) | ✅ **passed** — launched, stayed running | `package.yml` + `release.yml` smoke step |
| SHA-256 checksums | ✅ generated | `SHA256SUMS.txt` (release asset) |

### Windows build evidence (run 41982545548)

| Artifact | Size | SHA-256 |
| --- | --- | --- |
| `VYRONIX-0.1.0-x64-portable.exe` | 7,134,720 B | `d0e06b08530b013b89921fc5ef10ad2a5d0215efbc6626f82eb5c3577ffb1f98` |
| `VYRONIX-0.1.0-x64-setup.exe` | 2,687,348 B | `5a78712073b86a5d6d8b08f2ec58daba5018f89ef498f5c3cc6f2782da0f3df8` |

These are **real, compiled Windows binaries**, not placeholders. The portable exe was
launched on a Windows runner and remained running (no crash) for the smoke-test window.
(The sandbox cannot download the binaries — the artifact blob host is network-blocked —
but they exist as CI artifacts and are attached to the draft release.) The draft release
`v0.1.0` re-built and re-verified the same code; its `SHA256SUMS.txt` is authoritative for
the released files.

### Test coverage

- **Frontend (24):** formatting (unknown → "—", never fabricated); module registry/search;
  honest IPC unavailable constructors (browser throws `NativeUnavailableError`, no mocks);
  measurement rendering (available/unavailable/estimated; Unknown ≠ Good; Permission Required).
- **Core (72):** battery health/wear math & edge cases; SMART ATA/NVMe interpretation &
  thresholds; weighted scoring with unsupported-exclusion & confidence; diagnostics rules;
  recommendations; report serialization/escaping; validation helpers.

## Level C — Physical device validation (PENDING — owner performed)

**Not yet recorded.** The owner will perform these tests manually on real Windows 10 and
Windows 11 computers using [`MANUAL_HARDWARE_TESTING.md`](MANUAL_HARDWARE_TESTING.md).
Results are added below **only after** the owner personally runs the tests and submits a
report (via the “Hardware test report” issue template or email). No physical result is
pre-filled or assumed.

When results arrive, record them in this format:

```
### <device make/model> — Windows <edition/version/build> — <date>
- Tester: <name> · VYRONIX: 0.1.0 (<portable/installer>) · admin: <yes/no>
- Per-module verdicts: <PASS/FAIL/BLOCKED/LIMITATION summary>
- Compatibility matrix: <link to filled matrix>
- Bugs/limitations: <links to issues>
- Evidence: <screenshots/exports>
```

Pending coverage: real WMI/sysinfo/battery/storage readings; interactive display/keyboard/
mouse/audio tests; benchmark/stress on real CPUs; peripheral & port inspection (camera, USB,
HDMI, Bluetooth, etc.); installer + portable launch on clean Windows machines.

## How to reproduce (Levels A & B)

```bash
# Core
cargo test -p vyronix-core
cargo clippy -p vyronix-core --all-targets -- -D warnings
cargo fmt --all -- --check

# Frontend (apps/desktop)
npm ci && ./node_modules/.bin/tsc --noEmit && ./node_modules/.bin/vitest run && ./node_modules/.bin/vite build

# Backend (Windows CI gate)
cargo check --workspace
```

## Integrity rule

If a check cannot be run in this environment, it is marked **pending**, never "passed".
Physical-device claims are made **only** from the owner's submitted, observed results. This
protects the acceptance criterion: no claim of success or compatibility without evidence.
