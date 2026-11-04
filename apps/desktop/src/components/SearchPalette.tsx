import { useEffect, useRef, useState } from "react";
import { Search, CornerDownLeft } from "lucide-react";
import { searchIndex, type SearchEntry } from "../lib/modules";
import { useAppStore } from "../lib/store";

export function SearchPalette() {
  const open = useAppStore((s) => s.searchOpen);
  const setOpen = useAppStore((s) => s.setSearchOpen);
  const setPage = useAppStore((s) => s.setPage);
  const [query, setQuery] = useState("");
  const [selected, setSelected] = useState(0);
  const inputRef = useRef<HTMLInputElement>(null);

  const results = searchIndex(query);

  useEffect(() => {
    if (open) {
      setQuery("");
      setSelected(0);
      setTimeout(() => inputRef.current?.focus(), 0);
    }
  }, [open]);

  useEffect(() => { setSelected(0); }, [query]);

  if (!open) return null;

  const go = (entry: SearchEntry) => {
    setPage(entry.page);
    setOpen(false);
  };

  const onKey = (e: React.KeyboardEvent) => {
    if (e.key === "ArrowDown") { e.preventDefault(); setSelected((s) => Math.min(s + 1, results.length - 1)); }
    else if (e.key === "ArrowUp") { e.preventDefault(); setSelected((s) => Math.max(s - 1, 0)); }
    else if (e.key === "Enter" && results[selected]) { go(results[selected]); }
    else if (e.key === "Escape") { setOpen(false); }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-start justify-center pt-24 px-4 animate-fade-in" role="dialog" aria-modal="true" aria-label="Global search">
      <div className="absolute inset-0 bg-black/50 backdrop-blur-sm" onClick={() => setOpen(false)} />
      <div className="relative vx-panel w-full max-w-xl overflow-hidden animate-slide-up">
        <div className="flex items-center gap-3 px-4 py-3 border-b border-[var(--vx-border)]">
          <Search className="w-4 h-4 text-[var(--vx-muted)]" />
          <input
            ref={inputRef}
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            onKeyDown={onKey}
            placeholder="Search modules, tools and settings…"
            className="flex-1 bg-transparent outline-none text-sm"
            aria-label="Search"
          />
          <kbd className="text-[10px] px-1.5 py-0.5 rounded bg-[var(--vx-border)] text-[var(--vx-muted)]">Esc</kbd>
        </div>
        <div className="max-h-80 overflow-y-auto py-2">
          {query && results.length === 0 && (
            <div className="px-4 py-6 text-center text-sm text-[var(--vx-muted)]">No results for “{query}”</div>
          )}
          {!query && (
            <div className="px-4 py-6 text-center text-sm text-[var(--vx-muted)]">
              Type to search across all diagnostic modules, tools and settings.
            </div>
          )}
          {results.map((r, i) => (
            <button
              key={r.id}
              onClick={() => go(r)}
              onMouseEnter={() => setSelected(i)}
              className={`w-full flex items-center gap-3 px-4 py-2.5 text-left transition ${i === selected ? "bg-vx-600/15" : ""}`}
            >
              <div className="min-w-0 flex-1">
                <div className="text-sm font-medium">{r.title}</div>
                <div className="text-xs text-[var(--vx-muted)] truncate">{r.description}</div>
              </div>
              <span className="text-[10px] uppercase tracking-wider text-[var(--vx-muted)]">{r.kind}</span>
              {i === selected && <CornerDownLeft className="w-3.5 h-3.5 text-vx-500" />}
            </button>
          ))}
        </div>
      </div>
    </div>
  );
}
