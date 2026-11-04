import type { Severity, Status } from "../lib/types";
import { SEVERITY_LABELS, STATUS_LABELS } from "../lib/types";

const SEVERITY_STYLES: Record<Severity, string> = {
  good: "bg-emerald-500/15 text-emerald-500 border-emerald-500/30",
  information: "bg-sky-500/15 text-sky-500 border-sky-500/30",
  warning: "bg-amber-500/15 text-amber-500 border-amber-500/30",
  critical: "bg-red-500/15 text-red-500 border-red-500/30",
  unknown: "bg-slate-500/15 text-slate-400 border-slate-500/30",
};

const STATUS_STYLES: Record<Status, string> = {
  available: "bg-emerald-500/15 text-emerald-500 border-emerald-500/30",
  verified: "bg-emerald-500/15 text-emerald-500 border-emerald-500/30",
  estimated: "bg-sky-500/15 text-sky-500 border-sky-500/30",
  unsupported: "bg-slate-500/15 text-slate-400 border-slate-500/30",
  permission_required: "bg-amber-500/15 text-amber-500 border-amber-500/30",
  unavailable: "bg-slate-500/15 text-slate-400 border-slate-500/30",
  failed: "bg-red-500/15 text-red-500 border-red-500/30",
  not_tested: "bg-slate-500/15 text-slate-400 border-slate-500/30",
  manual_verification_required: "bg-violet-500/15 text-violet-400 border-violet-500/30",
};

export function SeverityBadge({ severity }: { severity: Severity }) {
  return (
    <span className={`inline-flex items-center rounded-full border px-2.5 py-0.5 text-xs font-semibold ${SEVERITY_STYLES[severity]}`}>
      {SEVERITY_LABELS[severity]}
    </span>
  );
}

export function StatusBadge({ status }: { status: Status }) {
  return (
    <span className={`inline-flex items-center rounded-full border px-2 py-0.5 text-[11px] font-medium ${STATUS_STYLES[status]}`}>
      {STATUS_LABELS[status]}
    </span>
  );
}

export function severityDot(severity: Severity): string {
  switch (severity) {
    case "good": return "bg-emerald-500";
    case "information": return "bg-sky-500";
    case "warning": return "bg-amber-500";
    case "critical": return "bg-red-500";
    default: return "bg-slate-500";
  }
}
