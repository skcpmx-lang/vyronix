import { ShieldCheck, Cpu, BatteryCharging, HardDrive, ArrowRight, Activity } from "lucide-react";
import { useAppStore } from "../lib/store";
import { VxLogo } from "../components/Sidebar";
import { isNative } from "../lib/ipc";

export function Welcome() {
  const setPage = useAppStore((s) => s.setPage);
  const setWelcomeSeen = useAppStore((s) => s.setWelcomeSeen);

  const enter = () => {
    setWelcomeSeen(true);
    setPage("dashboard");
  };

  return (
    <div className="min-h-full flex items-center justify-center p-8 bg-gradient-to-br from-vx-950 via-[#0a1120] to-[#071018] text-white">
      <div className="max-w-3xl w-full animate-slide-up">
        <div className="flex items-center gap-4 mb-8">
          <VxLogo size={64} />
          <div>
            <div className="text-xs tracking-[0.35em] text-accent font-semibold">MERQO SOFTWARE PRODUCT</div>
            <h1 className="text-5xl font-bold tracking-tight mt-1">VYRONIX</h1>
            <p className="text-lg text-vx-200 mt-2">Know Your Hardware. Trust Your Device.</p>
          </div>
        </div>

        <p className="text-vx-100/80 leading-relaxed mb-8 max-w-2xl">
          A premium, privacy-first diagnostic ecosystem for Windows PCs and laptops. VYRONIX reads real
          hardware through Windows interfaces, reports every value with its source and confidence, and
          never fabricates results. Unknown is never shown as good.
        </p>

        <div className="grid grid-cols-2 md:grid-cols-4 gap-3 mb-8">
          {[
            { icon: Cpu, label: "Hardware insight" },
            { icon: BatteryCharging, label: "Battery health" },
            { icon: HardDrive, label: "Storage SMART" },
            { icon: Activity, label: "Live monitoring" },
          ].map(({ icon: Icon, label }) => (
            <div key={label} className="rounded-xl border border-white/10 bg-white/5 px-4 py-3 flex items-center gap-3">
              <Icon className="w-5 h-5 text-accent" />
              <span className="text-sm">{label}</span>
            </div>
          ))}
        </div>

        <div className="flex items-center gap-3 mb-6 text-sm text-vx-100/70">
          <ShieldCheck className="w-5 h-5 text-emerald-400" />
          Offline-first. No telemetry, no ads, no hidden data collection. All diagnostics run locally.
        </div>

        {!isNative && (
          <div className="rounded-xl border border-amber-400/30 bg-amber-400/10 px-4 py-3 text-sm text-amber-200 mb-6">
            You are viewing the browser preview. Hardware features require the VYRONIX Windows desktop
            application; no hardware data is simulated here.
          </div>
        )}

        <button onClick={enter} className="vx-btn-primary text-base px-6 py-3">
          Enter VYRONIX <ArrowRight className="w-4 h-4" />
        </button>
        <div className="text-xs text-vx-100/40 mt-6">
          Founded by Shohan Khan · merqoonline@gmail.com · Windows 10 / 11 (64-bit)
        </div>
      </div>
    </div>
  );
}
