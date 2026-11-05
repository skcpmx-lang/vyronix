# VYRONIX Hardware Test Report

> Copy this file, fill it in on a real Windows 10/11 device, and submit it via a
> **“Hardware test report”** GitHub issue or email **merqoonline@gmail.com**.
> Record only what you actually observed. See `docs/MANUAL_HARDWARE_TESTING.md`.

## 1. Device identity
| Field | Value |
| --- | --- |
| Device name | |
| Manufacturer / model | |
| Motherboard (if known) | |
| CPU | |
| GPU(s) | |
| RAM (total / type) | |
| Storage (SSD/HDD/NVMe, model) | |
| Battery present? (laptop/desktop) | |
| Windows edition / version / build | |
| System architecture (x64) | |
| Form factor (laptop / desktop / AIO) | |
| Ran as administrator? (yes/no) | |

## 2. Test environment
| Field | Value |
| --- | --- |
| VYRONIX version (About page) | 0.1.0 |
| Build used (portable / installer) | |
| SHA-256 verified? (yes/no) | |
| Test date (UTC) | |
| Tester name / handle | |

## 3. Per-module results
Verdict: **PASS / FAIL / BLOCKED / LIMITATION**. Attach a screenshot per module.

| # | Module | Verdict | Evidence (screenshot/export) | Notes (what you saw) |
| --- | --- | --- | --- | --- |
| 1 | System Overview | | | |
| 2 | Processor (CPU) | | | |
| 3 | Graphics (GPU) | | | |
| 4 | Memory (RAM) | | | |
| 5 | Battery | | | |
| 6 | Storage + SMART | | | |
| 7 | Live Monitoring | | | |
| 8 | Display Test | | | |
| 9 | Keyboard Test | | | |
| 10 | Mouse & Touchpad | | | |
| 11 | Speaker Test | | | |
| 12 | Microphone Test | | | |
| 13 | CPU Benchmark | | | |
| 14 | Memory Benchmark | | | |
| 15 | Storage Benchmark | | | |
| 16 | Memory Test | | | |
| 17 | Stress Test | | | |
| 18 | Windows Diagnostics | | | |
| 19 | Network (ping/DNS) | | | |
| 20 | Device Inspection | | | |
| 21 | Dashboard score | | | |
| 22 | Reports (PDF/HTML/CSV/JSON) | | | |
| 23 | Settings / Search / About | | | |

## 4. Hardware compatibility matrix (this device)
Result: **PASS / FAIL / BLOCKED / LIMITATION**. Manual-only items are inspected physically.

| Component | Method | Result | Evidence / notes |
| --- | --- | --- | --- |
| CPU | Automated + benchmark | | |
| GPU | Automated | | |
| RAM | Automated + tests | | |
| SSD | Automated + benchmark | | |
| HDD | Automated + benchmark | | |
| NVMe | Automated + benchmark | | |
| Battery | Automated | | |
| Display | Interactive + visual | | |
| Keyboard | Interactive | | |
| Touchpad | Interactive | | |
| Audio (speaker) | Interactive | | |
| Microphone | Interactive | | |
| Camera | **Manual-only** | | |
| USB | **Manual-only** | | |
| HDMI | **Manual-only** | | |
| Wi-Fi | Automated + ping/DNS | | |
| Bluetooth | **Manual-only** | | |
| Ethernet | Automated + ping/DNS | | |
| Sensors (temps) | Automated (real sensors) | | |
| Windows diagnostics | Automated (read-only) | | |

## 5. Failures / bugs found
For each, give: module, steps to reproduce, expected vs actual, screenshot, reproducible?

1. …

## 6. Limitations observed (Unsupported / manual-only)
List values shown as `Unsupported` / `Permission Required` / `Unavailable` and whether that
matches the documented limitation.

1. …

## 7. Overall impression / additional notes
…

## 8. Attachments
- [ ] Screenshots (per module)
- [ ] Exported report (JSON preferred, plus PDF/HTML/CSV)
- [ ] `msinfo32` / `dxdiag` / Task Manager comparisons (if used)
