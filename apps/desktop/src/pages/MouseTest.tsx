import { useRef, useState } from "react";
import { Mouse, RotateCcw } from "lucide-react";
import { Card, PageHeader } from "../components/Card";

interface ButtonState { down: boolean; count: number; }

export function MouseTest() {
  const [active, setActive] = useState(false);
  const [buttons, setButtons] = useState<Record<number, ButtonState>>({
    0: { down: false, count: 0 }, 1: { down: false, count: 0 }, 2: { down: false, count: 0 },
  });
  const [scrollEvents, setScrollEvents] = useState(0);
  const [scrollDelta, setScrollDelta] = useState(0);
  const [moveCount, setMoveCount] = useState(0);
  const [position, setPosition] = useState({ x: 0, y: 0 });
  const [touches, setTouches] = useState(0);
  const areaRef = useRef<HTMLDivElement>(null);

  const reset = () => {
    setButtons({ 0: { down: false, count: 0 }, 1: { down: false, count: 0 }, 2: { down: false, count: 0 } });
    setScrollEvents(0); setScrollDelta(0); setMoveCount(0); setTouches(0);
  };

  const onPointerDown = (e: React.PointerEvent) => {
    if (!active) return;
    setButtons((b) => ({ ...b, [e.button]: { down: true, count: (b[e.button]?.count ?? 0) + 1 } }));
    if (e.pointerType === "touch") setTouches((t) => t + 1);
  };
  const onPointerUp = (e: React.PointerEvent) => {
    if (!active) return;
    setButtons((b) => ({ ...b, [e.button]: { ...b[e.button], down: false } }));
  };
  const onPointerMove = (e: React.PointerEvent) => {
    if (!active) return;
    setMoveCount((c) => c + 1);
    const rect = areaRef.current?.getBoundingClientRect();
    if (rect) setPosition({ x: Math.round(e.clientX - rect.left), y: Math.round(e.clientY - rect.top) });
  };
  const onWheel = (e: React.WheelEvent) => {
    if (!active) return;
    setScrollEvents((c) => c + 1);
    setScrollDelta((d) => d + e.deltaY);
  };

  const buttonCard = (label: string, btn: number) => {
    const st = buttons[btn] ?? { down: false, count: 0 };
    return (
      <div className={`rounded-lg border p-4 text-center transition ${st.down ? "border-vx-500 bg-vx-500/20" : "border-[var(--vx-border)]"}`}>
        <div className="text-sm font-medium">{label}</div>
        <div className="text-2xl font-bold tabular-nums mt-1">{st.count}</div>
        <div className="text-[11px] text-[var(--vx-muted)]">{st.down ? "Pressed" : "clicks"}</div>
      </div>
    );
  };

  return (
    <div>
      <PageHeader title="Mouse & Touchpad Test" description="Button, scroll, movement and gesture verification"
        actions={
          <div className="flex gap-2">
            <button className="vx-btn-ghost" onClick={reset}><RotateCcw className="w-4 h-4" /> Reset</button>
            <button className={active ? "vx-btn-ghost" : "vx-btn-primary"} onClick={() => setActive((a) => !a)}>
              <Mouse className="w-4 h-4" /> {active ? "Stop test" : "Start test"}
            </button>
          </div>
        } />

      <div className="grid grid-cols-3 gap-3 mb-4">
        {buttonCard("Left click", 0)}
        {buttonCard("Middle click", 1)}
        {buttonCard("Right click", 2)}
      </div>

      <div className="grid grid-cols-2 md:grid-cols-4 gap-3 mb-4">
        <div className="vx-panel p-4 text-center"><div className="text-2xl font-bold tabular-nums">{scrollEvents}</div><div className="text-xs text-[var(--vx-muted)]">Scroll events</div></div>
        <div className="vx-panel p-4 text-center"><div className="text-2xl font-bold tabular-nums">{Math.round(scrollDelta)}</div><div className="text-xs text-[var(--vx-muted)]">Total scroll Δ</div></div>
        <div className="vx-panel p-4 text-center"><div className="text-2xl font-bold tabular-nums">{moveCount}</div><div className="text-xs text-[var(--vx-muted)]">Move events</div></div>
        <div className="vx-panel p-4 text-center"><div className="text-2xl font-bold tabular-nums">{touches}</div><div className="text-xs text-[var(--vx-muted)]">Touch taps</div></div>
      </div>

      <Card title="Interaction area" subtitle={active ? `Pointer at (${position.x}, ${position.y}) — click, scroll and move here` : "Start the test, then interact in this area"}>
        <div
          ref={areaRef}
          onPointerDown={onPointerDown}
          onPointerUp={onPointerUp}
          onPointerMove={onPointerMove}
          onWheel={onWheel}
          onContextMenu={(e) => active && e.preventDefault()}
          className={`h-64 rounded-lg border-2 border-dashed flex items-center justify-center transition select-none
            ${active ? "border-vx-500/50 bg-vx-500/5 cursor-crosshair" : "border-[var(--vx-border)]"}`}
        >
          <div className="text-center text-sm text-[var(--vx-muted)]">
            {active ? (
              <>
                <div className="w-3 h-3 rounded-full bg-vx-500 mx-auto mb-2" style={{ transform: `translate(${position.x - 150}px, ${position.y - 120}px)`, transition: "transform 0.05s" }} />
                Click, scroll and move within this area
              </>
            ) : "Test inactive"}
          </div>
        </div>
        <p className="text-[11px] text-[var(--vx-muted)] mt-3">
          Touchpad gestures (two-finger scroll, multi-touch) are delivered by Windows as pointer/wheel events and are
          detected here where the driver exposes them. Gesture-level verification that requires driver-specific features
          is marked as manual verification. This is device-level input detection, not a guarantee of every gesture.
        </p>
      </Card>
    </div>
  );
}
