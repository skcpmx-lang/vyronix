// Formatting helpers. Pure and unit-testable.

export function formatBytes(bytes: number | null | undefined, decimals = 1): string {
  if (bytes === null || bytes === undefined || !isFinite(bytes)) return "—";
  if (bytes === 0) return "0 B";
  const units = ["B", "KB", "MB", "GB", "TB", "PB"];
  const i = Math.min(Math.floor(Math.log(Math.abs(bytes)) / Math.log(1024)), units.length - 1);
  const value = bytes / Math.pow(1024, i);
  return `${value.toFixed(i === 0 ? 0 : decimals)} ${units[i]}`;
}

export function formatMwh(mwh: number | null | undefined): string {
  if (mwh === null || mwh === undefined || !isFinite(mwh)) return "—";
  if (mwh >= 1000) return `${(mwh / 1000).toFixed(2)} Wh`;
  return `${Math.round(mwh)} mWh`;
}

export function formatPct(value: number | null | undefined, decimals = 0): string {
  if (value === null || value === undefined || !isFinite(value)) return "—";
  return `${value.toFixed(decimals)}%`;
}

export function formatTemp(c: number | null | undefined): string {
  if (c === null || c === undefined || !isFinite(c)) return "—";
  return `${c.toFixed(0)} °C`;
}

export function formatFreq(mhz: number | null | undefined): string {
  if (mhz === null || mhz === undefined || !isFinite(mhz)) return "—";
  if (mhz >= 1000) return `${(mhz / 1000).toFixed(2)} GHz`;
  return `${Math.round(mhz)} MHz`;
}

export function formatDuration(secs: number | null | undefined): string {
  if (secs === null || secs === undefined || !isFinite(secs)) return "—";
  const s = Math.floor(secs);
  const d = Math.floor(s / 86400);
  const h = Math.floor((s % 86400) / 3600);
  const m = Math.floor((s % 3600) / 60);
  const sec = s % 60;
  if (d > 0) return `${d}d ${h}h ${m}m`;
  if (h > 0) return `${h}h ${m}m ${sec}s`;
  if (m > 0) return `${m}m ${sec}s`;
  return `${sec}s`;
}

export function formatHours(hours: number | null | undefined): string {
  if (hours === null || hours === undefined || !isFinite(hours)) return "—";
  const h = Math.floor(hours);
  const m = Math.round((hours - h) * 60);
  return `${h}h ${m}m`;
}

export function formatNumber(n: number | null | undefined): string {
  if (n === null || n === undefined || !isFinite(n)) return "—";
  return n.toLocaleString("en-US");
}

export function formatMbps(bytesPerSec: number | null | undefined): string {
  if (bytesPerSec === null || bytesPerSec === undefined || !isFinite(bytesPerSec)) return "—";
  const mbps = (bytesPerSec * 8) / 1_000_000;
  if (mbps >= 1000) return `${(mbps / 1000).toFixed(2)} Gbps`;
  return `${mbps.toFixed(1)} Mbps`;
}

export function formatMBs(bytesPerSec: number | null | undefined): string {
  if (bytesPerSec === null || bytesPerSec === undefined || !isFinite(bytesPerSec)) return "—";
  return `${(bytesPerSec / (1024 * 1024)).toFixed(1)} MB/s`;
}

export function clamp(value: number, min: number, max: number): number {
  return Math.min(Math.max(value, min), max);
}

export function timestampToLocale(ms: number): string {
  if (!ms) return "—";
  return new Date(ms).toLocaleString();
}
