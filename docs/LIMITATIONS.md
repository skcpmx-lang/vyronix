# VYRONIX Known Limitations

Honest, documented limits. Where a hard requirement cannot be met exactly, the closest
**safe** alternative is implemented and the gap is stated here (never hidden or faked).

## Hardware / sensor limits

| Area | Limitation | What VYRONIX does instead |
| --- | --- | --- |
| CPU temperature | Many Windows systems expose no CPU temp sensor without a kernel driver. | Reads a labelled `sysinfo` sensor if present; otherwise `Unsupported`. **Never inferred from usage.** |
| GPU utilization / temperature | No generic, driver-free Windows interface bundled. | Marked `Unsupported` (identity/driver/resolution still shown via WMI). |
| Battery cycle count | Only some battery firmwares report it. | `Available` when reported, else `Unsupported`. |
| Detailed ATA SMART (reallocated/pending/uncorrectable sectors) | Requires the admin-only `root\WMI` ATA pass-through. | Uses no-admin `MSFT_StorageReliabilityCounter` (wear/temp/hours/error counters); ATA detail `Unsupported` with a note. |
| NVMe available spare / unsafe shutdowns | Requires the NVMe health log (admin/vendor). | `Unsupported` unless exposed; core NVMe parser is ready for when raw logs are available. |
| Wi-Fi signal strength | Requires the WLAN API, not WMI. | `Unsupported`; link speed/connection still shown. |
| System-wide disk throughput | Not exposed by sysinfo. | Monitoring shows CPU/memory/network; disk throughput honestly absent. |

## Test limits

- **Memory test** is a userspace pattern test — it detects addressable faults under
  load but is **not** a firmware/OS-level exhaustive RAM test (e.g. Windows Memory
  Diagnostic). This is stated in the result.
- **Display / keyboard / mouse / audio** tests are interactive and rely on the user's
  senses/peripherals; they include honest "visual/manual inspection" disclaimers.
- **Benchmarks** are reproducible relative metrics (CPU = Mops/s), not comparable to
  vendor-specific synthetic scores.

## Platform / environment limits

- **Target is Windows 10/11 x64.** On other platforms the backend returns honest
  "unavailable" states (it compiles, but does not fabricate hardware data).
- **Development sandbox is Linux** with no Rust toolchain and no Windows device:
  - Rust is compiled via **GitHub Actions**, not locally.
  - **No physical-hardware readings have been observed** during development; on-device
    validation is a release gate and will be recorded when performed.
- **Unsigned builds** → Windows SmartScreen may warn on first run. Checksums are
  provided for integrity verification.

## Report / PDF limits

- PDF is generated client-side (jsPDF). Very large reports may paginate across many
  pages; complex scripts/RTL are limited to the fonts bundled.
- HTML/CSV/JSON come from the same validated model; all dynamic values are escaped.

## Why these limits are acceptable

Every limitation is surfaced to the user as an honest status (`Unsupported`,
`Permission Required`, `Unavailable`, `Manual Verification Required`) with a source and
explanation, rather than a fabricated value. This preserves the core promise:
**Know Your Hardware. Trust Your Device.**
