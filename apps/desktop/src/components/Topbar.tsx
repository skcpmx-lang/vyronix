import { Search, Sun, Moon, RefreshCw } from "lucide-react";
import { useAppStore } from "../lib/store";
import { moduleById } from "../lib/modules";

export function Topbar({ onRefresh }: { onRefresh?: () => void }) {
  const page = useAppStore((s) => s.page);
  const setSearchOpen = useAppStore((s) => s.setSearchOpen);
  const settings = useAppStore((s) => s.settings);
  const updateSettings = useAppStore((s) => s.updateSettings);
  const resolvedTheme = useAppStore((s) => s.resolvedTheme);
  const mod = moduleById(page);

  const toggleTheme = () => {
    updateSettings({ theme: resolvedTheme === "dark" ? "light" : "dark" });
  };

  return (
    <header className="h-16 shrink-0 flex items-center justify-between gap-4 px-6 border-b border-[var(--vx-border)] bg-[var(--vx-panel)]">
      <div className="min-w-0">
        <div className="text-sm font-semibold truncate">{mod?.title ?? "VYRONIX"}</div>
        <div className="text-[11px] text-[var(--vx-muted)] truncate">{mod?.description}</div>
      </div>

      <div className="flex items-center gap-2">
        <button
          onClick={() => setSearchOpen(true)}
          className="flex items-center gap-2 px-3 py-1.5 rounded-lg border border-[var(--vx-border)] text-xs text-[var(--vx-muted)] hover:bg-vx-500/10 transition min-w-[180px]"
        >
          <Search className="w-3.5 h-3.5" />
          <span className="flex-1 text-left">Search…</span>
          <kbd className="text-[10px] px-1.5 py-0.5 rounded bg-[var(--vx-border)]">Ctrl K</kbd>
        </button>

        {onRefresh && (
          <button onClick={onRefresh} className="p-2 rounded-lg hover:bg-vx-500/10 transition" aria-label="Refresh data" title="Refresh">
            <RefreshCw className="w-4 h-4" />
          </button>
        )}

        <button onClick={toggleTheme} className="p-2 rounded-lg hover:bg-vx-500/10 transition"
          aria-label={`Switch to ${resolvedTheme === "dark" ? "light" : "dark"} theme`}
          title={`Theme: ${settings.theme}`}>
          {resolvedTheme === "dark" ? <Sun className="w-4 h-4" /> : <Moon className="w-4 h-4" />}
        </button>
      </div>
    </header>
  );
}
