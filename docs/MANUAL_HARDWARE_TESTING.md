# VYRONIX Manual Hardware Testing Guide

**Version:** v0.1.0 · **Audience:** the owner / hardware tester · **Platform:** Windows 10/11 x64

This guide lets you validate VYRONIX on **real physical Windows computers**. It is the
authoritative procedure for **physical device validation**. Read the honesty rules first.

---

## 0. Honesty rules (read before testing)

1. **Record only what you actually observe.** Never fill in a result you did not see.
2. **Unknown is not Good.** If a value is missing, record the status VYRONIX shows
   (`Unsupported`, `Unavailable`, `Permission Required`, etc.) — do not guess.
3. **Estimated ≠ measured.** If VYRONIX labels a value `Estimated`, treat it as an
   estimate, not a sensor reading.
4. **Do not claim compatibility that was not verified.** A component is "validated on
   hardware" only after you personally observe it working on a real device and record it.
5. **A missing automated test is not a bug by itself.** Some components (camera, USB,
   HDMI, Bluetooth) are **manual-inspection only** in v0.1.0 — that is a documented
   limitation, not a defect. Log it as a limitation, not a failure, unless the manual
   inspection itself reveals a problem.

---

## 1. Validation levels — what is already proven vs. what you will prove

| Level | What it covers | Status | Who/where |
| --- | --- | --- | --- |
| **A — Automated** | Core unit tests (72), frontend unit tests (24), clippy, `rustfmt`, TypeScript typecheck, production build | ✅ **Done** | GitHub Actions (`ci.yml`) + sandbox |
| **B — Windows CI** | Backend compiles for `x86_64-pc-windows-msvc`; release build; NSIS + portable packaging; **executable launch smoke test**; SHA-256 checksums | ✅ **Done** | GitHub Actions (`backend.yml`, `package.yml`, `release.yml`) on `windows-latest` |
| **C — Physical device** | Real sensor readings, interactive tests, benchmarks/stress on actual hardware; peripheral & port inspection | ⏳ **You perform this** | Real Windows 10/11 PCs/laptops, using this guide |

**Key point:** Levels A and B prove the software *builds, launches and its logic is
correct*. They do **not** prove that every sensor/peripheral behaves correctly on every
machine. **Level C (this guide) is what establishes real-hardware compatibility.** Until
you record Level C results, no component is claimed as hardware-validated.

---

## 2. Preparation

### 2.1 What you need
- A **Windows 10 or Windows 11 x64** computer (the more variety, the better: different
  CPU/GPU vendors, SSD/HDD/NVMe, laptop vs desktop, with/without battery).
- The release artifacts: `VYRONIX-0.1.0-x64-portable.exe` **or** `VYRONIX-0.1.0-x64-setup.exe`,
  plus `SHA256SUMS.txt` (from the draft release).
- A way to capture evidence: screenshots (`Win+Shift+S`), and a notes file or the
  [test report template](templates/HARDWARE_TEST_REPORT_TEMPLATE.md).
- Optional peripherals to exercise ports/tests: USB drive, HDMI/DisplayPort monitor,
  headphones/mic, external mouse, Bluetooth device, Ethernet cable.
- **Administrator access is optional.** Most features work without it; a few data sources
  (TPM, detailed ATA SMART) will show `Permission Required` if not elevated.

### 2.2 Verify the download (integrity)
```powershell
Get-FileHash .\VYRONIX-0.1.0-x64-portable.exe -Algorithm SHA256
```
Compare the hash to `SHA256SUMS.txt`. **Do not run a binary whose hash does not match.**

### 2.3 Install / run
- **Portable:** double-click `VYRONIX-0.1.0-x64-portable.exe`. No installation needed.
- **Installer:** run `VYRONIX-0.1.0-x64-setup.exe` and follow the NSIS prompts (per-machine).
- **SmartScreen:** builds are **unsigned**. If SmartScreen warns, choose **More info →
  Run anyway** *only after* you verified the SHA-256 hash and trust the source.
- First launch opens the main window. If it does not open, see §8 Troubleshooting.

### 2.4 Safety
- VYRONIX is **read-only by default**. It does not modify the registry, install/remove
  drivers, change security settings, or reboot.
- **Storage benchmarks** write only to a **dedicated temp file** in a folder you choose,
  check free space first, and clean up. Point them at a folder with plenty of free space.
- **Stress tests** are bounded by duration and a temperature limit and can be stopped at
  any time. **Stay with the machine** during a stress test; stop it if anything sounds,
  smells or looks wrong. Temperature limits reduce risk but cannot guarantee safety.
- **Microphone test** shows a live level meter only — it never records or stores audio.
- **Keyboard test** counts key events but never logs what you type.

### 2.5 Capture the device identity first
Open **System Overview** and record: device name, manufacturer, model, motherboard, BIOS,
OS edition/version/build, architecture. This identifies the machine in your report. Take a
screenshot. Also note the VYRONIX version from **About** (should be 0.1.0).

---

## 3. How to record results

### 3.1 Status vocabulary (use exactly what VYRONIX shows)
`Available` · `Verified` · `Estimated` · `Unsupported` · `Permission Required` ·
`Unavailable` · `Failed` · `Not Tested` · `Manual Verification Required`.

### 3.2 Per-test verdict
For each test below assign one of:
- **PASS** — observed behaviour matches the expected behaviour and pass criteria.
- **FAIL** — observed behaviour contradicts the expected behaviour (a real defect).
- **BLOCKED** — could not run (missing hardware/permission); record why.
- **LIMITATION** — the feature is honestly `Unsupported`/manual-only; not a defect.

### 3.3 Evidence to collect (per test)
- A **screenshot** of the module showing the values/status.
- For reports: the exported **JSON** (most complete) and/or PDF.
- A short **note**: what you did, what you saw, verdict, and the device identity.
- For failures: exact steps to reproduce, what you expected, what happened, and any error
  text. A screenshot of the failure is very helpful.

### 3.4 Where to record
- Use the [test report template](templates/HARDWARE_TEST_REPORT_TEMPLATE.md) for a full
  device session, **or**
- Open a GitHub issue using the **“Hardware test report”** template (see §7), **or**
- Email **merqoonline@gmail.com** with the same information.

> **Important:** Physical validation is recorded in `docs/VALIDATION.md` **only after you
> personally perform the tests** and submit results. Do not pre-fill it.

---

## 4. Recommended test session order

1. System Overview → 2. Processor → 3. Graphics → 4. Memory → 5. Battery (laptops) →
6. Storage → 7. Live Monitoring → 8. Display → 9. Keyboard → 10. Mouse/Touchpad →
11. Speaker → 12. Microphone → 13. CPU benchmark → 14. Memory benchmark →
15. Storage benchmark → 16. Memory test → 17. Stress test (attended) →
18. Windows Diagnostics → 19. Network → 20. Device Inspection (ports/camera/etc.) →
21. Dashboard score → 22. Reports → 23. Settings/About.

Run interactive/peripheral tests with the relevant hardware connected. Repeat the whole
session on each distinct machine you want to validate.

---

## 5. Module-by-module procedures

> Each module lists: **Steps → Expected → Pass → Fail → Evidence → Limits → Troubleshooting.**
> Navigate using the left sidebar or global search (`Ctrl+K`).

### 5.1 System Overview
- **Steps:** Open **System Overview**. Read device/motherboard/BIOS/OS fields, Secure Boot,
  TPM, virtualization, uptime. Cross-check a few against Windows (`msinfo32`).
- **Expected:** Real identity values with `Available` status and a source (WMI/sysinfo).
  Secure Boot/TPM/virtualization show a real state or an honest `Unsupported`/`Permission
  Required`/`Unavailable`.
- **Pass:** Device/manufacturer/model/OS match `msinfo32`; no fabricated values.
- **Fail:** Values contradict `msinfo32`, or a value appears that the machine cannot have.
- **Evidence:** Screenshot + note of any field that disagrees with `msinfo32`.
- **Limits:** TPM may need admin (`Permission Required`). Some firmware fields are not exposed.
- **Troubleshooting:** If many fields are `Unavailable`, run once **as administrator** and
  re-check; note whether elevation changed anything.

### 5.2 Processor (CPU)
- **Steps:** Open **Processor**. Note name, cores/threads, base/current/max clock, usage,
  temperature. Compare name/core count with Task Manager → Performance → CPU.
- **Expected:** Correct CPU name and core/thread counts; live usage; temperature only if a
  real sensor is exposed (otherwise `Unsupported`).
- **Pass:** Name/cores/threads match Task Manager; usage responds to load.
- **Fail:** Wrong CPU identity or core count; usage stuck at 0 under load.
- **Evidence:** Screenshot + Task Manager comparison.
- **Limits:** **CPU temperature is never inferred from usage.** If no sensor is exposed it is
  `Unsupported` — that is a limitation, not a failure.
- **Troubleshooting:** If temperature is `Unsupported`, that is expected on many systems
  without a supported sensor interface. Record it as a limitation.

### 5.3 Graphics (GPU)
- **Steps:** Open **Graphics**. Note adapter name(s), driver version/date, dedicated memory,
  resolution, refresh rate. Compare with `dxdiag` → Display.
- **Expected:** Real adapter identity/driver/resolution from WMI. Utilization/temperature are
  `Unsupported` (no generic interface bundled).
- **Pass:** Adapter name/driver/resolution match `dxdiag`.
- **Fail:** Wrong adapter identity or driver version.
- **Evidence:** Screenshot + `dxdiag` comparison.
- **Limits:** GPU utilization & temperature are `Unsupported` in v0.1.0 (documented). Hybrid
  graphics may list both iGPU + dGPU.
- **Troubleshooting:** If a dGPU is missing, note whether it appears in `dxdiag`; report as a
  bug only if `dxdiag` shows it but VYRONIX does not.

### 5.4 Memory (RAM)
- **Steps:** Open **Memory**. Note total/used/available and per-DIMM capacity/speed/type/slot.
  Compare total with Task Manager and slot info with `msinfo32`/vendor spec.
- **Expected:** Correct total RAM; per-DIMM details where SMBIOS exposes them.
- **Pass:** Total matches Task Manager; DIMM count/capacity plausible for the machine.
- **Fail:** Total RAM wrong; DIMM data clearly contradicts the physical configuration.
- **Evidence:** Screenshot + Task Manager total.
- **Limits:** Some DIMM fields (speed/type/slot) may be `Unavailable` if SMBIOS omits them.
- **Troubleshooting:** If DIMM details are empty but total is correct, record as a limitation.

### 5.5 Battery (laptops only)
- **Steps:** Open **Battery**. Note design vs full-charge capacity, health %, wear %, cycle
  count, charge %, charging status, rate, runtime estimate. Compare with
  `powercfg /battery-report` (run in an admin terminal) if available.
- **Expected:** Real capacities; **health/wear computed from design vs full-charge capacity**;
  cycle count only if firmware reports it. On a desktop, "no battery" is shown honestly.
- **Pass:** Health/wear are consistent with the capacity ratio; charging status matches the
  actual AC/battery state (plug/unplug the charger and watch it change).
- **Fail:** Health shown when capacities are missing; charging status does not change when
  you plug/unplug AC; fabricated cycle count.
- **Evidence:** Screenshot at least once on battery and once charging; note cycle-count status.
- **Limits:** Cycle count is `Unsupported` on many batteries. Runtime is an **estimate**.
- **Troubleshooting:** If no battery is detected on a laptop, report as a bug (include model).

### 5.6 Storage (SSD / HDD / NVMe) + SMART
- **Steps:** Open **Storage**. For each drive note model, capacity, media type (SSD/HDD),
  interface, health, temperature, power-on hours, wear, error counters, partitions. Compare
  model/capacity with `msinfo32` → Components → Storage and Disk Management.
- **Expected:** Correct drive identity/capacity; SMART-derived health from the Windows Storage
  interface (no admin needed). Partitions mapped to the right drive.
- **Pass:** Drive model/capacity/media type match the physical drive; partitions correct.
- **Fail:** Wrong capacity/model; a drive missing that Disk Management shows; partitions
  attributed to the wrong drive.
- **Evidence:** Screenshot per drive + Disk Management comparison.
- **Limits:** Detailed ATA attributes (reallocated/pending/uncorrectable sectors) and NVMe
  available-spare/unsafe-shutdowns need the admin pass-through and are `Unsupported` without
  it. Wear is typically reported for SSDs, not HDDs.
- **Troubleshooting:** If SMART fields are `Unsupported`, try running once as admin and note
  any change; otherwise record as a limitation.

### 5.7 Live Monitoring (sensors)
- **Steps:** Open **Live Monitoring**. Watch CPU %, memory %, network throughput, and any
  temperatures. Generate load (open apps, copy a file, browse) and confirm the graphs react.
- **Expected:** CPU/memory update in real time; network rates respond to traffic; temperatures
  only from real sensors.
- **Pass:** Graphs move and reflect actual activity; no flatlined/fake values under load.
- **Fail:** Values frozen under changing load; a temperature shown with no sensor.
- **Evidence:** Short screen recording or 2–3 screenshots (idle vs load).
- **Limits:** GPU utilization and system-wide disk throughput are not exposed (absent, not
  faked). Some machines expose no temperature sensors.
- **Troubleshooting:** If nothing updates, note the monitoring interval in Settings and report.

### 5.8 Display Test
- **Steps:** Open **Display Test**, start it, and (optionally) go fullscreen. Step through all
  **15 patterns** (Red, Green, Blue, White, Black, Gray 50%, Yellow, Cyan, Magenta, Color
  gradient, Grayscale ramp, Checkerboard, Pixel grid, Contrast bars, Uniformity). Inspect the
  panel for dead/stuck pixels, backlight bleed, uniformity and color issues. Use ←/→ to
  navigate, Esc to exit.
- **Expected:** Each pattern fills the screen cleanly; fullscreen works; navigation works.
- **Pass:** All patterns render; you can inspect the panel; controls work.
- **Fail:** A pattern fails to render, fullscreen breaks, or navigation is unresponsive.
- **Evidence:** Note any panel defects you see (these are *device* defects, not app bugs) and
  whether the app rendered patterns correctly.
- **Limits:** This is a **visual inspection aid** — the app cannot detect dead pixels for you;
  you must look. Multi-monitor selection may be limited.
- **Troubleshooting:** If fullscreen is blocked, note any OS/browser fullscreen restriction.

### 5.9 Keyboard Test
- **Steps:** Open **Keyboard Test**. Press every key (including modifiers, function row,
  arrows, numpad). Watch the on-screen map highlight each key and counters increment. Check
  for keys that do not register or appear "stuck".
- **Expected:** Each physical key registers and is shown; counters increase; no key content is
  logged.
- **Pass:** All keys register correctly; no false "stuck" keys.
- **Fail:** A working key does not register; a key shows stuck when it is not.
- **Evidence:** Screenshot of the map after pressing all keys; list any key that misbehaves.
- **Limits:** Uses standard key events; some special/vendor keys may not map. No global hook.
- **Troubleshooting:** If a key never registers, test it in Notepad to confirm whether it is a
  hardware fault vs an app mapping issue; report accordingly.

### 5.10 Mouse & Touchpad Test
- **Steps:** Open **Mouse & Touchpad**. Click left/right/middle, scroll the wheel, move the
  pointer in the interaction area; on a laptop use the touchpad (move, tap, two-finger scroll).
  Watch counters for buttons/wheel/move/touch.
- **Expected:** Button clicks, wheel and movement are counted; touchpad input registers.
- **Pass:** All buttons and scroll register; pointer/touch movement tracked.
- **Fail:** A button/scroll does not register; touchpad not detected on a laptop.
- **Evidence:** Screenshot of counters; note any gesture that does not register.
- **Limits:** Advanced multi-finger gestures may not be fully captured (documented).
- **Troubleshooting:** Test the same action in Windows to distinguish hardware vs app issues.

### 5.11 Audio (Speaker) Test
- **Steps:** Open **Audio & Microphone** → speaker section. Play the test tone; switch
  left/right/stereo; adjust frequency (e.g. 440 Hz). Listen. Try headphones if available.
- **Expected:** A tone plays in the selected channel(s); frequency change is audible.
- **Pass:** You hear the tone in the correct channel(s); controls work.
- **Fail:** No sound when the system volume is up and the correct output is selected.
- **Evidence:** Note which channels produced sound; screenshot of the controls.
- **Limits:** Uses Web Audio; relies on the OS default output device.
- **Troubleshooting:** Confirm the Windows output device/volume first; if other apps play
  sound but VYRONIX does not, report as a bug.

### 5.12 Microphone Test
- **Steps:** Open **Audio & Microphone** → microphone section. Grant mic permission. Speak and
  watch the **live level (RMS) meter** move. Confirm no recording is stored.
- **Expected:** The meter responds to your voice; permission prompt appears; nothing is recorded.
- **Pass:** Meter moves with speech; stops when you stop.
- **Fail:** Meter never moves with a working mic and granted permission.
- **Evidence:** Screenshot of the meter while speaking; note the input device.
- **Limits:** Level meter only — no recording/playback of your voice (by design).
- **Troubleshooting:** Check Windows mic privacy settings and the selected input device; if
  other apps see the mic but VYRONIX does not, report as a bug.

### 5.13 CPU Benchmark
- **Steps:** Open **Benchmarks** → CPU. Choose a duration (e.g. 5–10 s) and run. Note
  single-core and multi-core scores (labelled **Mops/s**) and ops/sec.
- **Expected:** The test runs for the chosen duration and returns real measured scores;
  multi-core ≥ single-core on a multi-core CPU.
- **Pass:** Scores produced; multi-core generally higher; repeat runs are in a similar range.
- **Fail:** Test errors out, returns 0, or multi-core < single-core on a multi-core CPU.
- **Evidence:** Screenshot of results; note CPU model and duration.
- **Limits:** Scores are a reproducible relative metric (Mops/s), not comparable to vendor
  synthetic benchmarks.
- **Troubleshooting:** If scores vary wildly, close background apps and re-run; note thermal
  throttling if the machine is hot.

### 5.14 Memory Benchmark
- **Steps:** Open **Benchmarks** → Memory. Choose a size (128/256/512 MiB) and run. Note
  read/write/copy bandwidth and latency.
- **Expected:** Real bandwidth/latency numbers produced for the chosen size.
- **Pass:** Bandwidth/latency returned and plausible for the RAM type.
- **Fail:** Error, or zero/negative bandwidth.
- **Evidence:** Screenshot; note RAM type/size.
- **Limits:** Userspace measurement; affected by other memory activity.
- **Troubleshooting:** Close memory-heavy apps and re-run for a cleaner number.

### 5.15 Storage Benchmark
- **Steps:** Open **Benchmarks** → Storage. Choose a folder with **plenty of free space** and a
  size (64–512 MiB). Confirm the safety prompt. Run. Note sequential read/write and random
  4K IOPS/latency. Verify the temp file is removed afterwards.
- **Expected:** A dedicated temp file is created, measured, and **deleted**; real speeds returned.
- **Pass:** Speeds returned; temp file cleaned up; no other files touched.
- **Fail:** Temp file left behind; error despite sufficient free space; any write outside the
  chosen folder.
- **Evidence:** Screenshot of results + confirmation the temp file is gone; note drive type.
- **Limits:** File-based benchmark (not raw device); results depend on the filesystem/cache.
- **Troubleshooting:** If it reports insufficient space, choose a folder/drive with more free
  space. **Never** point it at a folder you cannot afford to have a temp file in.

### 5.16 Memory Test
- **Steps:** Open **Benchmarks** (or Memory) → Memory test. Choose a size and run. Note patterns
  tested, bytes tested, errors found, pass/fail.
- **Expected:** Patterns (zeros/ones/checkerboards/walking-ones/address) written and verified;
  errors counted honestly.
- **Pass:** Test completes; on healthy RAM, 0 errors.
- **Fail:** Test crashes; or reports errors (which may indicate real RAM faults — re-run to confirm).
- **Evidence:** Screenshot of the result; if errors, note the first error offset and re-run.
- **Limits:** **Userspace pattern test, not a firmware-level exhaustive RAM test.** For a
  definitive RAM diagnosis use Windows Memory Diagnostic.
- **Troubleshooting:** If errors appear, re-run; persistent errors suggest hardware — confirm
  with Windows Memory Diagnostic before concluding.

### 5.17 Stress Test (attended)
- **Steps:** Open **Stress Test**. Set a duration (start with 60 s) and a temperature limit
  (default 90 °C). Read the safety warning, confirm, and start. **Stay with the machine.** Watch
  CPU usage/temperature and the status. Try **Pause** and **Stop**. Let one run finish by duration.
- **Expected:** CPU load rises; status updates; pause/stop work; the test stops at the duration or
  if the temperature limit is reached.
- **Pass:** Load applied; controls work; automatic stop at duration/temperature works.
- **Fail:** Cannot stop; ignores the temperature limit; system becomes unstable and the app does
  not stop.
- **Evidence:** Screenshot of status during the run; note max temperature seen and stop reason.
- **Limits:** Temperature-based safety depends on a sensor being exposed; if no sensor, the
  duration limit still applies. **Never leave a stress test unattended.**
- **Troubleshooting:** If the machine overheats or behaves oddly, press **Stop** immediately.

### 5.18 Windows Diagnostics
- **Steps:** Open **Windows Diagnostics**. Browse the tabs: Processes, Services, Drivers,
  Startup, Critical/Error events. Apply a filter. Compare a few entries with Task Manager and
  `services.msc`.
- **Expected:** Real lists populate (read-only); problem drivers/events are surfaced; row caps apply.
- **Pass:** Lists match the system (spot-check); no modification is made to the system.
- **Fail:** Empty lists on a system that has these items; incorrect data.
- **Evidence:** Screenshot of each tab; note any obviously wrong entry.
- **Limits:** Event-log query uses a read-only `Get-WinEvent`; large logs are capped.
- **Troubleshooting:** If a tab is empty, note whether the corresponding Windows tool shows data.

### 5.19 Network (Wi-Fi / Ethernet / Bluetooth adapters, ping, DNS)
- **Steps:** Open **Network**. Note adapters (name, type, MAC, IP, link speed, connected, DNS).
  Run **Ping** (e.g. `1.1.1.1`, 4 packets) and **DNS** (e.g. `example.com`). Test on Wi-Fi and,
  if available, Ethernet.
- **Expected:** Real adapter info; ping returns received/loss and min/avg/max RTT; DNS resolves
  to real addresses.
- **Pass:** Adapter matches `ipconfig`; ping/DNS succeed on a working connection.
- **Fail:** Wrong adapter info; ping/DNS fail on a known-good connection.
- **Evidence:** Screenshot + `ipconfig` comparison; note connection type.
- **Limits:** Wi-Fi **signal strength** is `Unsupported` (needs WLAN API). Bluetooth may appear
  as an adapter but has **no dedicated functionality test** in v0.1.0.
- **Troubleshooting:** If ping fails, confirm the internet works in a browser first; note any
  firewall that may block ICMP.

### 5.20 Device Inspection (29-item checklist) — ports, camera, physical
- **Steps:** Open **Device Inspection**. Work through all 29 items across the categories (Core
  hardware, Storage, Power, Display & input, Audio, Ports & connectivity, Cooling & power,
  Physical condition). For items that map to a module (CPU/GPU/RAM/SSD/HDD/battery), use that
  module's result. For physical items, **inspect the hardware directly** and set the status +
  notes. Save.
- **Expected:** You can set each item's status and notes; the summary counters update; it persists.
- **Pass:** All items recordable; summary reflects your entries; persists across restart.
- **Fail:** Status does not save; counters wrong; data lost on restart.
- **Evidence:** Screenshot of the completed checklist; export via Reports (inspection report).
- **Limits:** **Camera, USB, HDMI, DisplayPort, audio jack, cooling/fans, and physical items are
  manual-inspection only** in v0.1.0 — there is no automated test for them. This is a documented
  limitation.
- **How to inspect the manual items:**
  - **Camera:** open the Windows Camera app; confirm a live image. Record pass/fail in the checklist.
  - **USB ports:** plug a USB drive into each port; confirm it is detected in File Explorer.
  - **HDMI / DisplayPort:** connect an external monitor; confirm a signal (Win+P → Extend).
  - **Audio jack:** plug headphones; confirm sound output.
  - **Bluetooth:** pair a Bluetooth device in Windows Settings; confirm it connects.
  - **Cooling/fans:** listen/feel for airflow under load; check for abnormal noise.
  - **Physical/hinges/chassis/screws/charging port:** visual/tactile inspection.

### 5.21 Dashboard / Diagnostic engine & scoring
- **Steps:** Open **Dashboard**. Run the diagnostics/scan. Review the overall score, coverage,
  confidence, per-component scores, findings and recommendations.
- **Expected:** A transparent weighted score over **measurable** components only; unsupported
  items excluded; findings cite evidence and source; recommendations match the hardware.
- **Pass:** Score/coverage/confidence are consistent with what is actually measurable; no
  unsupported item counted as healthy.
- **Fail:** A score appears when nothing is measurable; an unsupported item inflates the score;
  findings contradict the data.
- **Evidence:** Screenshot of the dashboard + the exported report (JSON) for the same run.
- **Limits:** The score reflects only what the machine exposes; low coverage → low confidence.
- **Troubleshooting:** If coverage is low, note which components are `Unsupported` and why.

### 5.22 Reports (PDF / HTML / CSV / JSON)
- **Steps:** Open **Reports**. Choose a report type (full/hardware/battery/storage/benchmark/
  inspection/summary) and a format. Toggle "include sensitive data" off, then on, and note the
  difference. Generate & export each format (PDF, HTML, CSV, JSON). Open each file.
- **Expected:** All four formats generate and open; content matches the collected data; sensitive
  identifiers (serial/MAC/IP) appear **only** when the toggle is on.
- **Pass:** Each format opens and is readable; sensitive-data toggle works; no fabricated values.
- **Fail:** A format fails to generate/open; sensitive data leaks when the toggle is off.
- **Evidence:** Keep one exported file of each format; note any rendering issue.
- **Limits:** PDF is generated offline (jsPDF); very large reports paginate; complex scripts limited.
- **Troubleshooting:** If save fails, check the destination folder permissions and the overwrite
  prompt.

### 5.23 Settings / Themes / Search / About
- **Steps:** Open **Settings**. Switch dark/light theme; toggle animations; change monitoring
  interval; change default report format and sensitive-data default. Use global search (`Ctrl+K`)
  to jump to a module. Open **About** and confirm version/branding.
- **Expected:** Theme/animations apply; settings persist across restart; search finds modules;
  About shows VYRONIX 0.1.0 / MERQO / owner info.
- **Pass:** Settings persist; theme switches; search works; About correct.
- **Fail:** Settings do not persist; theme does not apply; search broken.
- **Evidence:** Screenshot of each theme; note persistence after restart.
- **Limits:** —
- **Troubleshooting:** If settings reset, note whether the app data folder is writable.

---

## 6. Hardware compatibility test matrix

Use one row per component **per device**. "Method" states honestly how VYRONIX covers it in
v0.1.0. **Manual-only** means there is no automated test — you inspect it physically (§5.20).

| Component | VYRONIX coverage (v0.1.0) | Method | What to verify | Pass criteria |
| --- | --- | --- | --- | --- |
| **CPU** | Processor page + benchmark | Automated collection + benchmark | Identity, cores/threads, clocks, usage, temp (if exposed) | Matches Task Manager; benchmark runs |
| **GPU** | Graphics page | Automated collection | Adapter, driver, VRAM, resolution, refresh | Matches `dxdiag` (util/temp are `Unsupported`) |
| **RAM** | Memory page + benchmark + memory test | Automated collection + tests | Total, per-DIMM, bandwidth, pattern test | Total matches Task Manager; tests run |
| **SSD** | Storage page + benchmark | Automated collection (SMART) + benchmark | Model, capacity, health, wear, temp | Matches Disk Management; SMART honest |
| **HDD** | Storage page + benchmark | Automated collection (SMART) + benchmark | Model, capacity, health, power-on hours | Matches Disk Management; SMART honest |
| **NVMe** | Storage page + benchmark | Automated collection (SMART) + benchmark | Model, capacity, health, temp, wear | Matches Disk Management; spare/unsafe-shutdowns may be `Unsupported` |
| **Battery** | Battery page | Automated collection | Capacities, health/wear, cycles, charge state | Health consistent; charge state reacts to AC |
| **Display** | Display Test (15 patterns) | Interactive test + visual inspection | Patterns render; panel defects | All patterns render; you inspect panel |
| **Keyboard** | Keyboard Test | Interactive test | Every key registers | All keys register; no false stuck |
| **Touchpad** | Mouse & Touchpad Test | Interactive test | Move/tap/scroll | Registers on a laptop |
| **Audio (speaker)** | Audio Test | Interactive test | Tone L/R/stereo, frequency | Audible in correct channels |
| **Microphone** | Mic Test (live RMS meter) | Interactive test | Meter responds to voice | Meter moves; nothing recorded |
| **Camera** | Inspection checklist only | **Manual-only** | Live image via Windows Camera app | You confirm image; record in checklist |
| **USB** | Inspection checklist only | **Manual-only** | Each port detects a USB drive | You confirm detection; record in checklist |
| **HDMI** | Inspection checklist only | **Manual-only** | External monitor gets signal | You confirm signal; record in checklist |
| **Wi-Fi** | Network page | Automated collection + ping/DNS | Adapter, IP, connectivity | Matches `ipconfig`; ping/DNS work (signal `Unsupported`) |
| **Bluetooth** | May appear as adapter; inspection | **Manual-only** (no dedicated test) | Pair a device in Windows | You confirm pairing; record in checklist |
| **Ethernet** | Network page | Automated collection + ping/DNS | Adapter, link speed, IP, connectivity | Matches `ipconfig`; ping/DNS work |
| **Sensors (temps)** | Monitoring + CPU/Storage pages | Automated collection (only real sensors) | Temps from exposed sensors | Real sensors only; absent = `Unsupported` |
| **Windows diagnostics** | Windows Diagnostics page | Automated collection (read-only) | Processes/services/drivers/startup/events | Lists match system spot-checks |

> Also available in the inspection checklist: Mouse, DisplayPort, audio jack, cooling/fans,
> power adapter, physical condition, hinges, chassis, screws, charging port.

**Matrix fill-in:** for each device, copy the table and add a **Result** column
(PASS/FAIL/BLOCKED/LIMITATION) and an **Evidence/notes** column. Attach it to your report.

---

## 7. Reporting results and hardware-specific bugs

### 7.1 Submit a full test session (preferred)
1. Fill in the [test report template](templates/HARDWARE_TEST_REPORT_TEMPLATE.md).
2. Open a GitHub issue → choose **“Hardware test report”** → paste the completed template,
   attach screenshots/exports. **Or** email it to **merqoonline@gmail.com**.
3. Include the device identity (§2.5), Windows version, VYRONIX version, and per-test verdicts.

### 7.2 Report a hardware-specific bug
1. Open a GitHub issue → choose **“Hardware-specific bug”**.
2. Provide: device make/model, CPU/GPU/RAM/storage, Windows version, VYRONIX version, the
   module, **steps to reproduce**, **expected vs actual**, screenshots, and whether it is
   reproducible. Note if running as admin changes it.
3. If the issue is a missing/`Unsupported` value that is a **documented limitation**, label it
   as a limitation rather than a bug (see §0.5).

### 7.3 What happens next
- Your results are reviewed and, once confirmed, **recorded in `docs/VALIDATION.md`** under a
  "Physical device validation" section with the device identity and date. Only then is a
  component described as hardware-validated.
- Confirmed bugs are triaged and fixed in a later version; the fix is re-verified by CI
  (Levels A/B) and, where hardware-dependent, by a follow-up physical test.

---

## 8. General troubleshooting

| Symptom | Try |
| --- | --- |
| App won't launch | Verify SHA-256; check SmartScreen/AV quarantine; run as admin once; check Windows Event Viewer → Application for errors. |
| SmartScreen warning | Expected for unsigned builds; verify hash, then More info → Run anyway. |
| Many fields `Unavailable` | Run once as administrator; note what changes. Some firmware simply doesn't expose the data. |
| A sensor is `Unsupported` | Usually a documented limitation (no supported interface). Record as limitation, not bug. |
| Antivirus flags the exe | Unsigned heuristic detection can occur; verify the hash and submit a false-positive report to the AV vendor if needed. |
| Report won't save | Check destination folder permissions; try a different folder; check the overwrite prompt. |
| App data / settings lost | Confirm the app data folder (`%APPDATA%/com.merqo.vyronix` or similar) is writable. |

If a problem persists, capture a screenshot + the steps and report it via §7.2.

---

## 9. After testing

- Submit your report (§7). **Do not edit `docs/VALIDATION.md` yourself to claim validation** —
  it is updated only from confirmed, submitted results.
- Keep the release as a **draft pre-release** until the owner authorizes publication.
- Thank you — your physical-device results are what turn "builds and launches" into
  "validated on real hardware."
