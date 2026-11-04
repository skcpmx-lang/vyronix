import { MODULES, type ModuleDef } from "../lib/modules";
import { useAppStore, type PageId } from "../lib/store";
import { isNative } from "../lib/ipc";

const CATEGORY_ORDER: ModuleDef["category"][] = [
  "Overview", "Hardware", "Testing", "Performance", "System", "Tools", "Application",
];

export function Sidebar() {
  const page = useAppStore((s) => s.page);
  const setPage = useAppStore((s) => s.setPage);

  return (
    <aside className="w-60 shrink-0 h-full flex flex-col border-r border-[var(--vx-border)] bg-[var(--vx-panel)]">
      {/* Brand */}
      <button
        onClick={() => setPage("dashboard")}
        className="flex items-center gap-3 px-5 h-16 border-b border-[var(--vx-border)] text-left hover:bg-vx-500/5 transition"
      >
        <VxLogo />
        <div>
          <div className="text-sm font-bold tracking-wide leading-none">VYRONIX</div>
          <div className="text-[10px] text-[var(--vx-muted)] tracking-[0.25em] mt-1">MERQO</div>
        </div>
      </button>

      {/* Navigation */}
      <nav className="flex-1 overflow-y-auto py-3 px-3" aria-label="Main navigation">
        {CATEGORY_ORDER.map((cat) => {
          const mods = MODULES.filter((m) => m.category === cat);
          if (!mods.length) return null;
          return (
            <div key={cat} className="mb-3">
              <div className="px-2 mb-1 text-[10px] uppercase tracking-widest text-[var(--vx-muted)] font-semibold">{cat}</div>
              {mods.map((m) => (
                <NavButton key={m.id} module={m} active={page === m.id} onClick={() => setPage(m.id)} />
              ))}
            </div>
          );
        })}
      </nav>

      {/* Footer */}
      <div className="px-4 py-3 border-t border-[var(--vx-border)]">
        <div className="flex items-center gap-2 text-[11px] text-[var(--vx-muted)]">
          <span className={`w-2 h-2 rounded-full ${isNative ? "bg-emerald-500" : "bg-amber-500"}`} />
          {isNative ? "Native backend connected" : "Browser preview (no hardware)"}
        </div>
      </div>
    </aside>
  );
}

function NavButton({ module, active, onClick }: { module: ModuleDef; active: boolean; onClick: () => void }) {
  const Icon = module.icon;
  return (
    <button
      onClick={onClick}
      aria-current={active ? "page" : undefined}
      className={`w-full flex items-center gap-3 px-3 py-2 rounded-lg text-sm transition mb-0.5
        ${active
          ? "bg-vx-600/15 text-vx-500 font-semibold border border-vx-600/30"
          : "text-[var(--vx-text)] hover:bg-vx-500/10 border border-transparent"}`}
    >
      <Icon className={`w-4 h-4 shrink-0 ${active ? "text-vx-500" : "text-[var(--vx-muted)]"}`} />
      <span className="truncate">{module.title}</span>
    </button>
  );
}

/// Original VYRONIX brand mark: a stylized "V" pulse inside a hexagonal chip.
export function VxLogo({ size = 32 }: { size?: number }) {
  return (
    <svg width={size} height={size} viewBox="0 0 48 48" fill="none" aria-hidden="true">
      <defs>
        <linearGradient id="vxg" x1="0" y1="0" x2="48" y2="48">
          <stop offset="0%" stopColor="#3391ff" />
          <stop offset="100%" stopColor="#22d3ee" />
        </linearGradient>
      </defs>
      <path d="M24 3 42 13.5v21L24 45 6 34.5v-21L24 3z" fill="url(#vxg)" opacity="0.15" />
      <path d="M24 3 42 13.5v21L24 45 6 34.5v-21L24 3z" stroke="url(#vxg)" strokeWidth="2" fill="none" />
      <path d="M14 17l10 16 10-16" stroke="url(#vxg)" strokeWidth="3" strokeLinecap="round" strokeLinejoin="round" fill="none" />
      <path d="M18 24h4l2-4 3 7 2-3h3" stroke="#22d3ee" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round" fill="none" opacity="0.9" />
    </svg>
  );
}
