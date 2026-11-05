# VYRONIX Implementation Plan (22 phases)

Phases are executed in order. A phase is only marked **Validated** when its
acceptance evidence exists (tests pass, build succeeds, or a documented manual
verification). Status legend: ✅ Validated · 🔄 In progress · ⏳ Pending.

| # | Phase | Status | Evidence / notes |
| --- | --- | --- | --- |
| 1 | Repo inspection & access verification | ✅ | Public repo, ADMIN access confirmed; branch workflow verified. |
| 2 | Requirements analysis & traceability | ✅ | See `REQUIREMENTS_TRACEABILITY.md`. |
| 3 | Architecture & technology selection | ✅ | See `ARCHITECTURE.md`, `DECISIONS.md`. |
| 4 | Branding & custom icon | ✅ | Original VYRONIX mark generated (`scripts/generate_icons.py`); full icon set incl. `.ico`. No MERQO logo imitation. |
| 5 | Scaffolding & infrastructure | ✅ | Cargo workspace, Vite/React/Tailwind, SQLite, logging, CI scaffolding. |
| 6 | Core data model & honesty contract | ✅ | `Measurement<T>`, `Status`, `Severity`, `Confidence`; mirrored in TS. |
| 7 | Core battery / SMART / scoring / diagnostics + tests | ✅ | **72 unit tests pass; clippy clean** (CI). |
| 8 | Report model & serialization (core) | ✅ | JSON/HTML/CSV, escaping, UTC timestamps; unit-tested. |
| 9 | Frontend foundation (shell/theme/components/store) | ✅ | App shell, 12 shared components, zustand, CSS variables, reduced-motion. |
| 10 | System / CPU / GPU / motherboard / RAM pages | ✅ | Pages written; typecheck + build pass. Real collection in backend. |
| 11 | Battery diagnostics | ✅ | UI + validated health math; backend via starship-battery. |
| 12 | Storage SMART + benchmarks | ✅ | UI + backend SMART via Windows Storage ns; safe temp-file benchmark. |
| 13 | Real-time monitoring | ✅ | Monitoring page + persistent sysinfo collector. |
| 14 | Interactive tests (display/keyboard/mouse/audio/mic) | ✅ | 4 test pages; honest visual/manual disclaimers; no content capture. |
| 15 | Benchmarks & stress | ✅ | CPU/memory/storage benchmarks, memory test, bounded stress UI + backend. |
| 16 | Windows & network diagnostics | ✅ | Read-only tabs, ping, DNS; backend collectors. |
| 17 | Guided inspection (29 categories) | ✅ | 29-item checklist across 7 categories; persisted. |
| 18 | Diagnostic engine integration + scoring UI | ✅ | Dashboard + reports consume findings/score/recommendations. |
| 19 | Reports (PDF/HTML/CSV/JSON) + export | ✅ | Offline PDF (jsPDF) + backend bundle; save dialog + guarded write. |
| 20 | Premium UI polish (a11y, search, settings, animations) | ✅ | Global search palette, settings, themes, keyboard shortcuts. |
| 21 | Security / privacy / performance hardening + QA | ✅ (automated) | Core + frontend tests pass; backend Windows compile clean; physical QA is owner-performed (Level C). |
| 22 | Packaging, CI/CD, release gates, verified release | 🔄 | Portable + NSIS built, smoke test passed, **draft pre-release `v0.1.0` created**; publication awaits owner authorization. |

## Current focus

Automated (Level A) and Windows CI (Level B) validation are complete, and a **draft
pre-release `v0.1.0`** exists with the portable exe, installer and checksums. The remaining
steps are **owner-driven**: (1) perform manual hardware testing on real Windows 10/11 devices
using `MANUAL_HARDWARE_TESTING.md` and submit results, and (2) explicitly authorize publication
of the draft release. Physical-device testing does not block preparing the draft.

## Resumability

On `CONTINUE`, resume from the verified state: re-install `node_modules` (not persisted),
recover git HEAD with a **mixed** reset to the remote branch if an interruption reset it, check
the latest GitHub Actions runs for CI status, and continue the highest-priority unfinished
step. The temporary `.ci-*` diagnostic files have been removed; CI gates now fail directly on
error. Do not publish the draft release without explicit owner authorization.
