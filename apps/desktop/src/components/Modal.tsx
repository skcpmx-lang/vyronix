import { useEffect, type ReactNode } from "react";
import { X } from "lucide-react";

export function Modal({ open, onClose, title, children, footer, wide = false }: {
  open: boolean;
  onClose: () => void;
  title: string;
  children: ReactNode;
  footer?: ReactNode;
  wide?: boolean;
}) {
  useEffect(() => {
    if (!open) return;
    const onKey = (e: KeyboardEvent) => { if (e.key === "Escape") onClose(); };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, [open, onClose]);

  if (!open) return null;
  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-4 animate-fade-in" role="dialog" aria-modal="true" aria-label={title}>
      <div className="absolute inset-0 bg-black/50 backdrop-blur-sm" onClick={onClose} />
      <div className={`relative vx-panel w-full ${wide ? "max-w-3xl" : "max-w-lg"} max-h-[85vh] flex flex-col animate-slide-up`}>
        <header className="flex items-center justify-between px-5 py-4 border-b border-[var(--vx-border)]">
          <h2 className="text-base font-semibold">{title}</h2>
          <button onClick={onClose} className="p-1.5 rounded-lg hover:bg-vx-500/10 transition" aria-label="Close dialog">
            <X className="w-4 h-4" />
          </button>
        </header>
        <div className="px-5 py-4 overflow-y-auto">{children}</div>
        {footer && <footer className="px-5 py-4 border-t border-[var(--vx-border)] flex justify-end gap-2">{footer}</footer>}
      </div>
    </div>
  );
}

/// Confirmation dialog for destructive / high-load actions.
export function ConfirmDialog({ open, onClose, onConfirm, title, message, confirmLabel = "Confirm", danger = false }: {
  open: boolean;
  onClose: () => void;
  onConfirm: () => void;
  title: string;
  message: string;
  confirmLabel?: string;
  danger?: boolean;
}) {
  return (
    <Modal open={open} onClose={onClose} title={title}
      footer={
        <>
          <button className="vx-btn-ghost" onClick={onClose}>Cancel</button>
          <button className={danger ? "vx-btn bg-red-600 text-white hover:bg-red-500" : "vx-btn-primary"}
            onClick={() => { onConfirm(); onClose(); }}>
            {confirmLabel}
          </button>
        </>
      }>
      <p className="text-sm text-[var(--vx-muted)] leading-relaxed">{message}</p>
    </Modal>
  );
}
