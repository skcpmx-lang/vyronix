import type { ReactNode } from "react";

export function Card({ title, subtitle, actions, children, className = "" }: {
  title?: string;
  subtitle?: string;
  actions?: ReactNode;
  children: ReactNode;
  className?: string;
}) {
  return (
    <section className={`vx-panel p-5 animate-fade-in ${className}`}>
      {(title || actions) && (
        <header className="flex items-start justify-between gap-3 mb-4">
          <div>
            {title && <h3 className="text-base font-semibold">{title}</h3>}
            {subtitle && <p className="text-xs text-[var(--vx-muted)] mt-0.5">{subtitle}</p>}
          </div>
          {actions && <div className="flex items-center gap-2 shrink-0">{actions}</div>}
        </header>
      )}
      {children}
    </section>
  );
}

export function KeyValue({ label, value, mono = false }: { label: string; value: ReactNode; mono?: boolean }) {
  return (
    <div className="flex items-center justify-between gap-3 py-1.5 border-b border-[var(--vx-border)] last:border-0">
      <span className="text-xs text-[var(--vx-muted)]">{label}</span>
      <span className={`text-sm font-medium text-right ${mono ? "font-mono" : ""}`}>{value}</span>
    </div>
  );
}

export function PageHeader({ title, description, actions }: { title: string; description?: string; actions?: ReactNode }) {
  return (
    <div className="flex items-start justify-between gap-4 mb-6">
      <div>
        <h1 className="text-2xl font-bold tracking-tight">{title}</h1>
        {description && <p className="text-sm text-[var(--vx-muted)] mt-1 max-w-2xl">{description}</p>}
      </div>
      {actions && <div className="flex items-center gap-2 shrink-0">{actions}</div>}
    </div>
  );
}
