import { useCallback, useEffect, useState } from "react";
import { MonitorPlay, ChevronLeft, ChevronRight, X, Maximize } from "lucide-react";
import { Card, PageHeader } from "../components/Card";
import { NativeRequired } from "../components/NativeRequired";

interface Pattern { id: string; name: string; css: React.CSSProperties; hint: string; }

const SOLID = (color: string): React.CSSProperties => ({ background: color });

const PATTERNS: Pattern[] = [
  { id: "red", name: "Red", css: SOLID("#ff0000"), hint: "Look for dead/stuck pixels and color uniformity." },
  { id: "green", name: "Green", css: SOLID("#00ff00"), hint: "Look for dead/stuck pixels and color uniformity." },
  { id: "blue", name: "Blue", css: SOLID("#0000ff"), hint: "Look for dead/stuck pixels and color uniformity." },
  { id: "white", name: "White", css: SOLID("#ffffff"), hint: "Check for dark spots, dust and backlight bleeding at edges." },
  { id: "black", name: "Black", css: SOLID("#000000"), hint: "Check for bright/stuck pixels and backlight bleed in a dark room." },
  { id: "gray", name: "Gray (50%)", css: SOLID("#808080"), hint: "Check brightness uniformity across the panel." },
  { id: "yellow", name: "Yellow", css: SOLID("#ffff00"), hint: "Sub-pixel inspection (red+green)." },
  { id: "cyan", name: "Cyan", css: SOLID("#00ffff"), hint: "Sub-pixel inspection (green+blue)." },
  { id: "magenta", name: "Magenta", css: SOLID("#ff00ff"), hint: "Sub-pixel inspection (red+blue)." },
  { id: "gradient", name: "Color gradient", css: { background: "linear-gradient(90deg,#ff0000,#ffff00,#00ff00,#00ffff,#0000ff,#ff00ff,#ff0000)" }, hint: "Check for banding and smooth color transitions." },
  { id: "grayscale", name: "Grayscale ramp", css: { background: "linear-gradient(90deg,#000,#fff)" }, hint: "Check gray levels, banding and gamma." },
  { id: "checkerboard", name: "Checkerboard", css: { background: "repeating-conic-gradient(#000 0% 25%, #fff 0% 50%) 0 0 / 40px 40px" }, hint: "Check pixel alignment and sharpness." },
  { id: "pixelgrid", name: "Pixel grid", css: { background: "repeating-linear-gradient(0deg,#000 0 1px,#fff 1px 3px),repeating-linear-gradient(90deg,#000 0 1px,#fff 1px 3px)", backgroundBlendMode: "multiply" }, hint: "Fine grid for pixel-level inspection." },
  { id: "contrast", name: "Contrast bars", css: { background: "repeating-linear-gradient(90deg,#000 0 20px,#fff 20px 40px)" }, hint: "Check contrast and edge definition." },
  { id: "uniformity", name: "Uniformity (light gray)", css: SOLID("#d9d9d9"), hint: "Inspect for brightness/color non-uniformity and clouding." },
];

export function DisplayTest() {
  const [running, setRunning] = useState(false);
  const [index, setIndex] = useState(0);
  const [fullscreen, setFullscreen] = useState(false);

  const exit = useCallback(() => {
    setRunning(false);
    if (document.fullscreenElement) document.exitFullscreen().catch(() => {});
  }, []);

  const next = useCallback(() => setIndex((i) => (i + 1) % PATTERNS.length), []);
  const prev = useCallback(() => setIndex((i) => (i - 1 + PATTERNS.length) % PATTERNS.length), []);

  const start = async (fs: boolean) => {
    setIndex(0);
    setRunning(true);
    setFullscreen(fs);
    if (fs) {
      try { await document.documentElement.requestFullscreen(); } catch { /* fall back to overlay */ }
    }
  };

  useEffect(() => {
    if (!running) return;
    const onKey = (e: KeyboardEvent) => {
      if (e.key === "Escape") exit();
      else if (e.key === "ArrowRight" || e.key === " ") next();
      else if (e.key === "ArrowLeft") prev();
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, [running, exit, next, prev]);

  useEffect(() => {
    const onFsChange = () => { if (!document.fullscreenElement && running && fullscreen) exit(); };
    document.addEventListener("fullscreenchange", onFsChange);
    return () => document.removeEventListener("fullscreenchange", onFsChange);
  }, [running, fullscreen, exit]);

  const pattern = PATTERNS[index];

  return (
    <div>
      <PageHeader title="Display Test" description="Full-screen visual patterns for pixel, backlight and uniformity inspection" />
      <NativeRequired what="Multi-monitor selection" />

      <Card className="mb-4">
        <p className="text-sm text-[var(--vx-muted)] leading-relaxed mb-4">
          These are <strong>visual inspection aids</strong>, not automatic guarantees of physical pixel health.
          Cycle through solid colors, gradients and grids, and inspect the panel yourself. Use a dark room for
          black/bright-pixel checks. Navigate with <kbd className="px-1.5 py-0.5 rounded bg-[var(--vx-border)] text-xs">←</kbd>
          <kbd className="px-1.5 py-0.5 rounded bg-[var(--vx-border)] text-xs ml-1">→</kbd>, exit with
          <kbd className="px-1.5 py-0.5 rounded bg-[var(--vx-border)] text-xs ml-1">Esc</kbd>.
        </p>
        <div className="flex flex-wrap gap-2">
          <button className="vx-btn-primary" onClick={() => start(true)}>
            <Maximize className="w-4 h-4" /> Start full-screen test
          </button>
          <button className="vx-btn-ghost" onClick={() => start(false)}>
            <MonitorPlay className="w-4 h-4" /> Start in window
          </button>
        </div>
      </Card>

      <div className="grid grid-cols-3 sm:grid-cols-5 gap-2">
        {PATTERNS.map((p, i) => (
          <button key={p.id} onClick={() => { setIndex(i); setRunning(true); setFullscreen(false); }}
            className="rounded-lg border border-[var(--vx-border)] overflow-hidden hover:border-vx-500/50 transition">
            <div className="h-14 w-full" style={p.css} />
            <div className="text-[11px] py-1.5 px-1 truncate">{p.name}</div>
          </button>
        ))}
      </div>

      {running && (
        <div className="display-test-overlay" style={pattern.css} onClick={next}>
          {/* Control bar (auto-hides cursor via CSS) */}
          <div className="absolute top-0 left-0 right-0 flex items-center justify-between px-4 py-3 bg-black/60 text-white opacity-0 hover:opacity-100 transition-opacity">
            <div className="text-sm font-medium">{pattern.name} <span className="text-white/60">({index + 1}/{PATTERNS.length})</span></div>
            <div className="flex items-center gap-2">
              <button onClick={(e) => { e.stopPropagation(); prev(); }} className="p-2 rounded hover:bg-white/20" aria-label="Previous pattern"><ChevronLeft className="w-5 h-5" /></button>
              <button onClick={(e) => { e.stopPropagation(); next(); }} className="p-2 rounded hover:bg-white/20" aria-label="Next pattern"><ChevronRight className="w-5 h-5" /></button>
              <button onClick={(e) => { e.stopPropagation(); exit(); }} className="p-2 rounded hover:bg-white/20" aria-label="Exit test"><X className="w-5 h-5" /></button>
            </div>
          </div>
          <div className="absolute bottom-0 left-0 right-0 px-4 py-3 bg-black/60 text-white text-xs opacity-0 hover:opacity-100 transition-opacity">
            {pattern.hint} · Click or → for next · Esc to exit
          </div>
        </div>
      )}
    </div>
  );
}
