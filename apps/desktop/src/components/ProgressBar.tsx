import { clamp } from "../lib/format";

export function ProgressBar({ value, color, indeterminate = false, className = "" }: {
  value: number; // 0..100
  color?: string;
  indeterminate?: boolean;
  className?: string;
}) {
  const pct = clamp(value, 0, 100);
  return (
    <div className={`h-2 w-full rounded-full bg-[var(--vx-border)] overflow-hidden ${className}`}>
      {indeterminate ? (
        <div className="h-full w-1/3 rounded-full bg-vx-500 animate-[slideRight_1.2s_ease-in-out_infinite]"
          style={{ animationName: "vxIndeterminate" }} />
      ) : (
        <div className="h-full rounded-full transition-all duration-500"
          style={{ width: `${pct}%`, background: color ?? "#3391ff" }} />
      )}
      <style>{`@keyframes vxIndeterminate { 0% { transform: translateX(-100%);} 100% { transform: translateX(320%);} }`}</style>
    </div>
  );
}

export function Spinner({ size = 20 }: { size?: number }) {
  return (
    <svg width={size} height={size} viewBox="0 0 24 24" className="animate-spin text-vx-500">
      <circle cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="3" fill="none" opacity="0.2" />
      <path d="M12 2 a10 10 0 0 1 10 10" stroke="currentColor" strokeWidth="3" fill="none" strokeLinecap="round" />
    </svg>
  );
}
