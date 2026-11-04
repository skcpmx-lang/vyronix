import { CheckCircle2, Info, AlertTriangle, XCircle, X } from "lucide-react";
import { useAppStore } from "../lib/store";

const ICONS = {
  info: <Info className="w-4 h-4 text-sky-500" />,
  success: <CheckCircle2 className="w-4 h-4 text-emerald-500" />,
  warning: <AlertTriangle className="w-4 h-4 text-amber-500" />,
  error: <XCircle className="w-4 h-4 text-red-500" />,
};

export function Toasts() {
  const toasts = useAppStore((s) => s.toasts);
  const dismiss = useAppStore((s) => s.dismissToast);
  return (
    <div className="fixed bottom-4 right-4 z-[60] flex flex-col gap-2 max-w-sm">
      {toasts.map((t) => (
        <div key={t.id} className="vx-panel px-4 py-3 flex items-start gap-3 animate-slide-up shadow-card">
          <div className="mt-0.5 shrink-0">{ICONS[t.kind]}</div>
          <div className="min-w-0 flex-1">
            <div className="text-sm font-semibold">{t.title}</div>
            {t.message && <div className="text-xs text-[var(--vx-muted)] mt-0.5">{t.message}</div>}
          </div>
          <button onClick={() => dismiss(t.id)} className="p-1 rounded hover:bg-vx-500/10 shrink-0" aria-label="Dismiss notification">
            <X className="w-3.5 h-3.5" />
          </button>
        </div>
      ))}
    </div>
  );
}
