import { MonitorSmartphone } from "lucide-react";
import { isNative } from "../lib/ipc";

/// Honest notice shown in the browser preview where no native backend exists.
/// Never fabricates hardware data.
export function NativeRequired({ what }: { what: string }) {
  if (isNative) return null;
  return (
    <div className="vx-panel p-5 flex items-start gap-3 border-amber-500/30 bg-amber-500/5">
      <MonitorSmartphone className="w-5 h-5 text-amber-500 shrink-0 mt-0.5" />
      <div>
        <div className="text-sm font-semibold text-amber-500">Desktop runtime required</div>
        <p className="text-xs text-[var(--vx-muted)] mt-1 leading-relaxed">
          {what} can only be read by the VYRONIX Windows desktop application, which talks to real
          hardware through Windows APIs. This browser preview does not fabricate hardware data, so
          these fields are shown as unavailable. Build and run the Windows app to see live values.
        </p>
      </div>
    </div>
  );
}

export function EmptyState({ title, message, icon }: { title: string; message: string; icon?: React.ReactNode }) {
  return (
    <div className="vx-panel p-10 flex flex-col items-center justify-center text-center">
      {icon && <div className="mb-3 text-[var(--vx-muted)]">{icon}</div>}
      <div className="text-sm font-semibold">{title}</div>
      <p className="text-xs text-[var(--vx-muted)] mt-1 max-w-sm">{message}</p>
    </div>
  );
}
