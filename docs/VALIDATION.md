# VYRONIX Validation Record

This file records **actual evidence** of what has been validated. It is updated as
evidence arrives. No entry is added without a real, observed result.

## Automated (CI / local) — observed

| Item | Result | Where observed |
| --- | --- | --- |
| Core unit tests | **72 passed, 0 failed** | GitHub Actions, `.ci-diagnostics.txt` (`TEST_EXIT=0`) |
| Core clippy `-D warnings` | **0 warnings / errors** | `.ci-diagnostics.txt` (`CLIPPY_EXIT=0`) |
| Frontend `tsc --noEmit` | **0 errors** | local sandbox run |
| Frontend vitest | **24 passed (4 files)** | local sandbox run |
| Frontend Vite production build | **success** | local sandbox + Windows CI frontend step |
| Backend Windows compile | **in progress** — first pass found 4 errors, all fixed; re-check running | `.ci-backend-diagnostics.txt` |

### Frontend test coverage (24 tests)

- `format.test.ts` — byte/percent/duration/frequency/battery-unit formatting; unknown → "—" (never fabricated).
- `modules.test.ts` — module registry uniqueness, search index ranking/coverage.
- `ipc.test.ts` — honest unavailable constructors; browser mode throws `NativeUnavailableError` (no mock data).
- `MeasurementValue.test.tsx` — available/unavailable/estimated rendering; Unknown ≠ Good; Permission Required badge.

### Core test coverage (72 tests)

Battery health/wear math & edge cases; SMART ATA/NVMe interpretation & thresholds;
weighted scoring with unsupported-exclusion & confidence; diagnostics rules;
recommendations; report serialization/escaping; validation helpers.

## On-hardware — NOT yet observed

The development sandbox is Linux with no Windows device. The following are
**implemented but not yet validated on physical hardware** and will be recorded here
only after a real run:

- Actual WMI/sysinfo/battery/storage readings on a Windows 10/11 PC/laptop.
- Interactive display/keyboard/mouse/audio behaviour with real peripherals.
- Benchmark/stress behaviour and thermal safety on a real CPU.
- Installer + portable exe launch on a clean Windows machine (CI smoke pending).

## How to reproduce

```bash
# Core
cargo test -p vyronix-core
cargo clippy -p vyronix-core --all-targets -- -D warnings

# Frontend (apps/desktop)
npm ci && ./node_modules/.bin/tsc --noEmit && ./node_modules/.bin/vitest run && ./node_modules/.bin/vite build

# Backend (Windows CI)
cargo check --workspace   # output captured to .ci-backend-diagnostics.txt
```

## Integrity rule

If a check cannot be run in this environment, it is marked **pending**, never
"passed". This protects the acceptance criterion: no claim of success without evidence.
