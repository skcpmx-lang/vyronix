import type { Measurement } from "../lib/types";
import { StatusBadge } from "./StatusBadge";

interface Props<T> {
  label: string;
  measurement: Measurement<T>;
  format?: (value: T) => string;
  showStatus?: boolean;
  showSource?: boolean;
}

/// Renders an honest measurement: the value (or an em-dash), its status badge,
/// and the data source / note as a tooltip + optional footnote.
export function MeasurementValue<T>({ label, measurement, format, showStatus = true, showSource = false }: Props<T>) {
  const m = measurement;
  const hasValue = m.value !== null && m.value !== undefined;
  const display = hasValue ? (format ? format(m.value as T) : String(m.value)) : "—";
  const tooltip = [
    `Status: ${m.status}`,
    `Source: ${m.source}`,
    m.note ? `Note: ${m.note}` : null,
  ].filter(Boolean).join("\n");

  return (
    <div className="flex items-start justify-between gap-3 py-2" title={tooltip}>
      <div className="min-w-0">
        <div className="vx-label">{label}</div>
        {showSource && <div className="text-[11px] text-[var(--vx-muted)] truncate mt-0.5">{m.source}</div>}
        {m.note && m.status === "estimated" && (
          <div className="text-[11px] text-sky-500 mt-0.5">{m.note}</div>
        )}
      </div>
      <div className="flex flex-col items-end gap-1 shrink-0">
        <span className={`text-sm font-semibold tabular-nums ${hasValue ? "" : "text-[var(--vx-muted)]"}`}>
          {display}
        </span>
        {showStatus && <StatusBadge status={m.status} />}
      </div>
    </div>
  );
}
