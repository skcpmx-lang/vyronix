import { clamp } from "../lib/format";

/// Circular gauge for scores / percentages. SVG, no external dependency.
export function Gauge({ value, size = 120, label, sublabel, color }: {
  value: number | null;
  size?: number;
  label?: string;
  sublabel?: string;
  color?: string;
}) {
  const hasValue = value !== null && isFinite(value);
  const pct = hasValue ? clamp(value as number, 0, 100) : 0;
  const stroke = 10;
  const r = (size - stroke) / 2;
  const c = 2 * Math.PI * r;
  const offset = c - (pct / 100) * c;
  const autoColor = !hasValue
    ? "#64748b"
    : pct >= 80 ? "#10b981" : pct >= 60 ? "#0ea5e9" : pct >= 40 ? "#f59e0b" : "#ef4444";
  const strokeColor = color ?? autoColor;

  return (
    <div className="relative inline-flex items-center justify-center" style={{ width: size, height: size }}>
      <svg width={size} height={size} className="-rotate-90">
        <circle cx={size / 2} cy={size / 2} r={r} fill="none" stroke="var(--vx-border)" strokeWidth={stroke} />
        {hasValue && (
          <circle
            cx={size / 2} cy={size / 2} r={r} fill="none"
            stroke={strokeColor} strokeWidth={stroke} strokeLinecap="round"
            strokeDasharray={c} strokeDashoffset={offset}
            style={{ transition: "stroke-dashoffset 0.6s ease, stroke 0.3s ease" }}
          />
        )}
      </svg>
      <div className="absolute inset-0 flex flex-col items-center justify-center">
        <span className="text-2xl font-bold tabular-nums">{hasValue ? Math.round(value as number) : "—"}</span>
        {label && <span className="text-[10px] uppercase tracking-wider text-[var(--vx-muted)]">{label}</span>}
        {sublabel && <span className="text-[10px] text-[var(--vx-muted)]">{sublabel}</span>}
      </div>
    </div>
  );
}
