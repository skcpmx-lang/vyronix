import { useId } from "react";

/// Lightweight SVG area/line chart for live monitoring. No external dependency.
/// `data` is a rolling window of samples; null values render as gaps.
export function LiveChart({ data, max, height = 80, color = "#3391ff", unit = "" }: {
  data: (number | null)[];
  max?: number;
  height?: number;
  color?: string;
  unit?: string;
}) {
  const gradId = useId();
  const width = 300; // viewBox width; scales responsively
  const n = data.length;
  const effectiveMax = max ?? Math.max(1, ...data.filter((d): d is number => d !== null));
  const pad = 2;

  const points = data.map((v, i) => {
    if (v === null || !isFinite(v)) return null;
    const x = n <= 1 ? width / 2 : (i / (n - 1)) * (width - pad * 2) + pad;
    const y = height - pad - (Math.min(v, effectiveMax) / effectiveMax) * (height - pad * 2);
    return { x, y };
  });

  // Build path segments, breaking on nulls.
  let linePath = "";
  let areaPath = "";
  let started = false;
  let segmentStartX = 0;
  points.forEach((p) => {
    if (!p) { started = false; return; }
    if (!started) {
      linePath += `M ${p.x.toFixed(1)} ${p.y.toFixed(1)} `;
      areaPath += `M ${p.x.toFixed(1)} ${height - pad} L ${p.x.toFixed(1)} ${p.y.toFixed(1)} `;
      segmentStartX = p.x;
      started = true;
    } else {
      linePath += `L ${p.x.toFixed(1)} ${p.y.toFixed(1)} `;
      areaPath += `L ${p.x.toFixed(1)} ${p.y.toFixed(1)} `;
    }
    // close area segment lazily when next is null handled below
    void segmentStartX;
  });
  // Close the area path down to baseline.
  const lastValid = [...points].reverse().find((p) => p !== null);
  const firstValid = points.find((p) => p !== null);
  if (lastValid && firstValid) {
    areaPath += `L ${lastValid.x.toFixed(1)} ${height - pad} L ${firstValid.x.toFixed(1)} ${height - pad} Z`;
  }

  const current = [...data].reverse().find((d) => d !== null && isFinite(d));

  return (
    <div className="w-full">
      <svg viewBox={`0 0 ${width} ${height}`} className="w-full" style={{ height }} preserveAspectRatio="none">
        <defs>
          <linearGradient id={gradId} x1="0" y1="0" x2="0" y2="1">
            <stop offset="0%" stopColor={color} stopOpacity="0.35" />
            <stop offset="100%" stopColor={color} stopOpacity="0.02" />
          </linearGradient>
        </defs>
        {/* gridlines */}
        {[0.25, 0.5, 0.75].map((f) => (
          <line key={f} x1={pad} x2={width - pad} y1={height * f} y2={height * f}
            stroke="var(--vx-border)" strokeWidth="0.5" strokeDasharray="3 3" />
        ))}
        {areaPath && <path d={areaPath} fill={`url(#${gradId})`} />}
        {linePath && <path d={linePath} fill="none" stroke={color} strokeWidth="1.5" strokeLinejoin="round" />}
      </svg>
      <div className="flex justify-between text-[10px] text-[var(--vx-muted)] mt-1">
        <span>0</span>
        <span className="font-semibold tabular-nums" style={{ color }}>
          {current !== null && current !== undefined ? `${current.toFixed(unit === "%" ? 0 : 1)}${unit}` : "—"}
        </span>
        <span>{max !== undefined ? `${max}${unit}` : "auto"}</span>
      </div>
    </div>
  );
}
