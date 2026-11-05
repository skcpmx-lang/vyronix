# VYRONIX Security & Privacy

## Privacy commitments

- **Offline-first.** No telemetry, no analytics, no trackers, no accounts, no
  subscriptions, no paid APIs. The only network operations are **user-initiated**
  ping and DNS lookups.
- **Local persistence only.** Settings, inspection items and test history live in a
  local SQLite database under the app data directory. Logs are local and contain
  operational messages, not hardware identifiers or user content.
- **No secret recording.** The microphone test shows a **live level (RMS) meter
  only**; audio is never recorded or stored. The keyboard test counts key events but
  **never logs typed content**. No global keyboard hooks.
- **Sensitive identifiers gated.** Serial numbers, MAC and IP addresses are excluded
  from reports unless the user explicitly enables "include sensitive data".

## Least privilege & safe operations

- **Read-only by default.** Hardware collection uses read-only WMI/CIM, sysinfo and
  battery/storage queries. No system settings are changed.
- **No destructive storage writes.** Storage benchmarks create a **dedicated temp
  file** in a user-chosen folder, check free space first (1.5× guard), and guarantee
  cleanup. Raw device access is never used for writes.
- **No risky system changes.** No automatic reboot, no driver install/uninstall, no
  registry modification (Secure Boot is *read* via a fixed `reg query`), no disabling
  of Windows Defender / SmartScreen / security features.
- **Admin only when genuinely needed.** TPM namespace and detailed ATA SMART may
  require elevation; the app reports `Permission Required` rather than silently
  escalating or fabricating data.
- **Bounded stress.** Stress tests are limited by duration and a temperature ceiling
  and can be paused/stopped; they stop automatically if the limit is reached.

## Application security

- **Tauri capabilities are minimal:** `core:default`, `dialog:default` (save picker),
  `opener:default`. Hardware access is through **validated custom commands**, not
  broad plugin permissions.
- **Report file writes are guarded:** the save path is chosen via the native dialog;
  overwriting an existing file requires explicit consent; base64 payloads (PDF) are
  validated before writing.
- **HTML reports escape all dynamic values** to prevent injection.
- **Input validation:** benchmark sizes/durations are clamped; ping counts bounded;
  storage paths must be existing directories.
- **Unsigned builds.** No code-signing certificate is used (per constraint). Windows
  SmartScreen may warn on first run; this is disclosed in release notes. Users should
  verify SHA-256 checksums.

## Known hardening follow-ups (tracked, not blocking honesty)

- Set a strict Content-Security-Policy (currently `null` to guarantee the offline UI
  renders; the app loads only local bundled content, so exposure is minimal).
- Consider code-signing if a certificate becomes available (out of scope for v0.1).
- Add automated dependency auditing (`cargo audit`, `npm audit`) to CI.

## Reporting a security issue

Contact **merqoonline@gmail.com**. Please do not open public issues for sensitive
vulnerabilities before coordination.
