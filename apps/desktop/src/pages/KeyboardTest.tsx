import { useEffect, useRef, useState } from "react";
import { Keyboard, RotateCcw } from "lucide-react";
import { Card, PageHeader } from "../components/Card";

// Physical key rows (event.code based, layout-independent).
const ROWS: string[][] = [
  ["Escape", "F1", "F2", "F3", "F4", "F5", "F6", "F7", "F8", "F9", "F10", "F11", "F12"],
  ["Backquote", "Digit1", "Digit2", "Digit3", "Digit4", "Digit5", "Digit6", "Digit7", "Digit8", "Digit9", "Digit0", "Minus", "Equal", "Backspace"],
  ["Tab", "KeyQ", "KeyW", "KeyE", "KeyR", "KeyT", "KeyY", "KeyU", "KeyI", "KeyO", "KeyP", "BracketLeft", "BracketRight", "Backslash"],
  ["CapsLock", "KeyA", "KeyS", "KeyD", "KeyF", "KeyG", "KeyH", "KeyJ", "KeyK", "KeyL", "Semicolon", "Quote", "Enter"],
  ["ShiftLeft", "KeyZ", "KeyX", "KeyC", "KeyV", "KeyB", "KeyN", "KeyM", "Comma", "Period", "Slash", "ShiftRight"],
  ["ControlLeft", "MetaLeft", "AltLeft", "Space", "AltRight", "MetaRight", "ContextMenu", "ControlRight"],
];

const LABELS: Record<string, string> = {
  Escape: "Esc", Backquote: "`", Minus: "-", Equal: "=", Backspace: "⌫", Tab: "Tab",
  BracketLeft: "[", BracketRight: "]", Backslash: "\\", CapsLock: "Caps", Semicolon: ";",
  Quote: "'", Enter: "Enter", ShiftLeft: "Shift", Comma: ",", Period: ".", Slash: "/",
  ShiftRight: "Shift", ControlLeft: "Ctrl", MetaLeft: "Win", AltLeft: "Alt", Space: "Space",
  AltRight: "Alt", MetaRight: "Win", ContextMenu: "Menu", ControlRight: "Ctrl",
};

function label(code: string): string {
  if (LABELS[code]) return LABELS[code];
  if (code.startsWith("Key")) return code.slice(3);
  if (code.startsWith("Digit")) return code.slice(5);
  if (code.startsWith("F") && /^F\d+$/.test(code)) return code;
  return code;
}

export function KeyboardTest() {
  const [active, setActive] = useState(false);
  const [pressed, setPressed] = useState<Set<string>>(new Set());
  const [tested, setTested] = useState<Set<string>>(new Set());
  const [repeatCount, setRepeatCount] = useState(0);
  const [lastKey, setLastKey] = useState<string | null>(null);
  const stuckTimers = useRef<Map<string, ReturnType<typeof setTimeout>>>(new Map());
  const [stuck, setStuck] = useState<Set<string>>(new Set());

  useEffect(() => {
    if (!active) return;
    const down = (e: KeyboardEvent) => {
      e.preventDefault();
      const code = e.code;
      setLastKey(code);
      setPressed((p) => new Set(p).add(code));
      setTested((t) => new Set(t).add(code));
      if (e.repeat) setRepeatCount((c) => c + 1);
      // Stuck-key heuristic: if no keyup within 5s, flag for manual check.
      const existing = stuckTimers.current.get(code);
      if (existing) clearTimeout(existing);
      stuckTimers.current.set(code, setTimeout(() => {
        setStuck((s) => new Set(s).add(code));
      }, 5000));
    };
    const up = (e: KeyboardEvent) => {
      e.preventDefault();
      const code = e.code;
      setPressed((p) => { const n = new Set(p); n.delete(code); return n; });
      const t = stuckTimers.current.get(code);
      if (t) { clearTimeout(t); stuckTimers.current.delete(code); }
      setStuck((s) => { const n = new Set(s); n.delete(code); return n; });
    };
    window.addEventListener("keydown", down);
    window.addEventListener("keyup", up);
    return () => {
      window.removeEventListener("keydown", down);
      window.removeEventListener("keyup", up);
      stuckTimers.current.forEach((t) => clearTimeout(t));
      stuckTimers.current.clear();
    };
  }, [active]);

  const reset = () => {
    setPressed(new Set()); setTested(new Set()); setStuck(new Set());
    setRepeatCount(0); setLastKey(null);
  };

  const totalKeys = ROWS.flat().length;

  return (
    <div>
      <PageHeader title="Keyboard Test" description="Interactive key verification — presses are shown live, never recorded or transmitted"
        actions={
          <div className="flex gap-2">
            <button className="vx-btn-ghost" onClick={reset}><RotateCcw className="w-4 h-4" /> Reset</button>
            <button className={active ? "vx-btn-ghost" : "vx-btn-primary"} onClick={() => setActive((a) => !a)}>
              <Keyboard className="w-4 h-4" /> {active ? "Stop test" : "Start test"}
            </button>
          </div>
        } />

      <Card className="mb-4">
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4 text-center">
          <div><div className="text-2xl font-bold tabular-nums">{tested.size}</div><div className="text-xs text-[var(--vx-muted)]">Keys tested</div></div>
          <div><div className="text-2xl font-bold tabular-nums">{pressed.size}</div><div className="text-xs text-[var(--vx-muted)]">Currently pressed</div></div>
          <div><div className="text-2xl font-bold tabular-nums">{repeatCount}</div><div className="text-xs text-[var(--vx-muted)]">Repeat events</div></div>
          <div><div className="text-2xl font-bold tabular-nums">{stuck.size}</div><div className="text-xs text-[var(--vx-muted)]">Possible stuck</div></div>
        </div>
        {lastKey && <div className="text-center text-xs text-[var(--vx-muted)] mt-3">Last key: <span className="font-mono font-semibold text-vx-500">{lastKey}</span></div>}
        {!active && <p className="text-center text-sm text-[var(--vx-muted)] mt-3">Press <strong>Start test</strong>, then press keys on your keyboard. Each key lights up when detected.</p>}
      </Card>

      <Card title="Keyboard map" subtitle={`${tested.size} / ${totalKeys} keys detected · layout-independent physical positions`}>
        <div className="space-y-1.5">
          {ROWS.map((row, ri) => (
            <div key={ri} className="flex gap-1.5 flex-wrap">
              {row.map((code) => {
                const isPressed = pressed.has(code);
                const isTested = tested.has(code);
                const isStuck = stuck.has(code);
                return (
                  <div key={code}
                    className={`h-10 min-w-[38px] px-2 flex items-center justify-center rounded-md border text-xs font-medium transition-all
                      ${isStuck ? "bg-red-500/30 border-red-500 text-red-400"
                        : isPressed ? "bg-vx-500 border-vx-400 text-white scale-95"
                        : isTested ? "bg-emerald-500/20 border-emerald-500/50 text-emerald-400"
                        : "bg-[var(--vx-bg)] border-[var(--vx-border)] text-[var(--vx-muted)]"}`}
                    style={{ flexGrow: code === "Space" ? 6 : code === "Enter" || code === "Backspace" || code === "ShiftLeft" || code === "ShiftRight" ? 2 : 1 }}
                    title={code}>
                    {label(code)}
                  </div>
                );
              })}
            </div>
          ))}
        </div>
        <div className="flex gap-4 mt-4 text-xs text-[var(--vx-muted)]">
          <span className="flex items-center gap-1.5"><span className="w-3 h-3 rounded bg-emerald-500/40 border border-emerald-500" /> Tested</span>
          <span className="flex items-center gap-1.5"><span className="w-3 h-3 rounded bg-vx-500" /> Pressed now</span>
          <span className="flex items-center gap-1.5"><span className="w-3 h-3 rounded bg-red-500/40 border border-red-500" /> Possible stuck (verify manually)</span>
        </div>
        <p className="text-[11px] text-[var(--vx-muted)] mt-3">
          Detection uses standard Windows key events within this window only — no global keyboard hook is installed and
          typed content is never logged. Some special keys (media, brightness) are handled by the OS and may not reach the app.
          Stuck-key flags are heuristic; confirm physically.
        </p>
      </Card>
    </div>
  );
}
