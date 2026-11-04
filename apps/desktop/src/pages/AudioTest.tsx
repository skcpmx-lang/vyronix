import { useEffect, useRef, useState } from "react";
import { Volume2, Mic, Square, Play } from "lucide-react";
import { Card, PageHeader } from "../components/Card";
import { useAppStore } from "../lib/store";

interface AudioDevice { deviceId: string; label: string; kind: string; }

export function AudioTest() {
  const pushToast = useAppStore((s) => s.pushToast);
  const [devices, setDevices] = useState<AudioDevice[]>([]);
  const [playing, setPlaying] = useState<string | null>(null);
  const [frequency, setFrequency] = useState(440);
  const [micActive, setMicActive] = useState(false);
  const [micLevel, setMicLevel] = useState(0);
  const [micError, setMicError] = useState<string | null>(null);

  const ctxRef = useRef<AudioContext | null>(null);
  const oscRef = useRef<{ osc: OscillatorNode; gain: GainNode; panner: StereoPannerNode } | null>(null);
  const micStreamRef = useRef<MediaStream | null>(null);
  const micRafRef = useRef<number | null>(null);

  useEffect(() => {
    // Enumerate devices (labels may be empty until permission is granted).
    if (navigator.mediaDevices?.enumerateDevices) {
      navigator.mediaDevices.enumerateDevices().then((list) => {
        setDevices(list
          .filter((d) => d.kind === "audiooutput" || d.kind === "audioinput")
          .map((d) => ({ deviceId: d.deviceId, label: d.label || `${d.kind} (label hidden until permission)`, kind: d.kind })));
      }).catch(() => {});
    }
    return () => { stopOutput(); stopMic(); };
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const getCtx = () => {
    if (!ctxRef.current) ctxRef.current = new AudioContext();
    return ctxRef.current;
  };

  const stopOutput = () => {
    if (oscRef.current) {
      try { oscRef.current.osc.stop(); } catch { /* already stopped */ }
      oscRef.current.osc.disconnect();
      oscRef.current.gain.disconnect();
      oscRef.current.panner.disconnect();
      oscRef.current = null;
    }
    setPlaying(null);
  };

  const playTone = (pan: number, label: string) => {
    stopOutput();
    try {
      const ctx = getCtx();
      const osc = ctx.createOscillator();
      const gain = ctx.createGain();
      const panner = ctx.createStereoPanner();
      osc.type = "sine";
      osc.frequency.value = frequency;
      gain.gain.value = 0.15; // safe, moderate level
      panner.pan.value = pan;
      osc.connect(gain).connect(panner).connect(ctx.destination);
      osc.start();
      oscRef.current = { osc, gain, panner };
      setPlaying(label);
    } catch (e) {
      pushToast({ kind: "error", title: "Audio output unavailable", message: e instanceof Error ? e.message : String(e) });
    }
  };

  const stopMic = () => {
    if (micRafRef.current) cancelAnimationFrame(micRafRef.current);
    micStreamRef.current?.getTracks().forEach((t) => t.stop());
    micStreamRef.current = null;
    setMicActive(false);
    setMicLevel(0);
  };

  const startMic = async () => {
    setMicError(null);
    if (!navigator.mediaDevices?.getUserMedia) {
      setMicError("Microphone API is not available in this environment.");
      return;
    }
    try {
      // Explicit user action triggers the permission prompt. Nothing is recorded
      // or stored; we only read a live level meter.
      const stream = await navigator.mediaDevices.getUserMedia({ audio: true });
      micStreamRef.current = stream;
      const ctx = getCtx();
      const src = ctx.createMediaStreamSource(stream);
      const analyser = ctx.createAnalyser();
      analyser.fftSize = 1024;
      src.connect(analyser);
      const data = new Uint8Array(analyser.frequencyBinCount);
      setMicActive(true);
      const loop = () => {
        analyser.getByteTimeDomainData(data);
        let sum = 0;
        for (let i = 0; i < data.length; i++) { const v = (data[i] - 128) / 128; sum += v * v; }
        setMicLevel(Math.min(1, Math.sqrt(sum / data.length) * 3));
        micRafRef.current = requestAnimationFrame(loop);
      };
      loop();
      // Refresh device labels now that permission is granted.
      navigator.mediaDevices.enumerateDevices().then((list) => {
        setDevices(list.filter((d) => d.kind === "audiooutput" || d.kind === "audioinput")
          .map((d) => ({ deviceId: d.deviceId, label: d.label || d.kind, kind: d.kind })));
      }).catch(() => {});
    } catch (e) {
      setMicError(e instanceof Error ? e.message : "Microphone permission was denied or unavailable.");
    }
  };

  return (
    <div>
      <PageHeader title="Audio & Microphone" description="Speaker output and microphone input testing — permission-based, nothing uploaded" />

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
        <Card title="Speaker output test" subtitle="Generates real tones via the Web Audio API">
          <div className="mb-4">
            <label className="vx-label">Frequency: {frequency} Hz</label>
            <input type="range" min={100} max={2000} step={10} value={frequency}
              onChange={(e) => { setFrequency(Number(e.target.value)); if (oscRef.current) oscRef.current.osc.frequency.value = Number(e.target.value); }}
              className="w-full mt-2 accent-vx-500" aria-label="Test frequency" />
          </div>
          <div className="grid grid-cols-2 gap-2">
            <button className="vx-btn-ghost" onClick={() => playTone(-1, "left")}><Play className="w-4 h-4" /> Left channel</button>
            <button className="vx-btn-ghost" onClick={() => playTone(1, "right")}><Play className="w-4 h-4" /> Right channel</button>
            <button className="vx-btn-ghost" onClick={() => playTone(0, "stereo")}><Volume2 className="w-4 h-4" /> Stereo (both)</button>
            <button className="vx-btn-ghost" onClick={stopOutput} disabled={!playing}><Square className="w-4 h-4" /> Stop</button>
          </div>
          {playing && <div className="text-xs text-vx-500 mt-3">Playing: {playing} channel at {frequency} Hz</div>}
          <p className="text-[11px] text-[var(--vx-muted)] mt-3">
            Confirm you hear the tone in the expected speaker(s). Start at low system volume. This tests the output path;
            it does not measure acoustic quality.
          </p>
        </Card>

        <Card title="Microphone input test" subtitle="Live level meter only — no recording is retained">
          {!micActive ? (
            <button className="vx-btn-primary" onClick={startMic}><Mic className="w-4 h-4" /> Enable microphone</button>
          ) : (
            <button className="vx-btn-ghost" onClick={stopMic}><Square className="w-4 h-4" /> Stop microphone</button>
          )}
          {micError && <div className="text-xs text-red-500 mt-3">{micError}</div>}
          <div className="mt-4">
            <div className="vx-label mb-2">Input level</div>
            <div className="h-4 rounded-full bg-[var(--vx-border)] overflow-hidden">
              <div className="h-full rounded-full transition-all duration-75"
                style={{ width: `${micLevel * 100}%`, background: micLevel > 0.8 ? "#ef4444" : "#10b981" }} />
            </div>
            <div className="text-xs text-[var(--vx-muted)] mt-2">
              {micActive ? "Speak or make sound — the meter should respond." : "Enable the microphone to see the live level."}
            </div>
          </div>
          <p className="text-[11px] text-[var(--vx-muted)] mt-3">
            Your browser/OS will ask for microphone permission. VYRONIX only reads a live level meter here; audio is never
            recorded, stored or uploaded unless you explicitly use a recording feature and choose to save locally.
          </p>
        </Card>

        <Card title="Detected audio devices" className="lg:col-span-2">
          {devices.length === 0 ? (
            <p className="text-sm text-[var(--vx-muted)]">No audio devices enumerated yet. Device labels appear after microphone permission is granted.</p>
          ) : (
            <div className="grid grid-cols-1 md:grid-cols-2 gap-2">
              {devices.map((d) => (
                <div key={d.deviceId + d.kind} className="flex items-center gap-3 rounded-lg border border-[var(--vx-border)] px-3 py-2">
                  {d.kind === "audiooutput" ? <Volume2 className="w-4 h-4 text-vx-500" /> : <Mic className="w-4 h-4 text-emerald-500" />}
                  <div className="min-w-0">
                    <div className="text-sm truncate">{d.label}</div>
                    <div className="text-[11px] text-[var(--vx-muted)]">{d.kind === "audiooutput" ? "Output" : "Input"}</div>
                  </div>
                </div>
              ))}
            </div>
          )}
        </Card>
      </div>
    </div>
  );
}
