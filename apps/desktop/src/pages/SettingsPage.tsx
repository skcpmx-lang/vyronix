import { RotateCcw, Moon, Sun, Monitor } from "lucide-react";
import { Card, PageHeader } from "../components/Card";
import { useAppStore } from "../lib/store";
import { isNative } from "../lib/ipc";

export function SettingsPage() {
  const settings = useAppStore((s) => s.settings);
  const updateSettings = useAppStore((s) => s.updateSettings);
  const resetSettings = useAppStore((s) => s.resetSettings);
  const pushToast = useAppStore((s) => s.pushToast);

  return (
    <div>
      <PageHeader title="Settings" description="Preferences are validated and persisted safely"
        actions={<button className="vx-btn-ghost" onClick={() => { resetSettings(); pushToast({ kind: "success", title: "Settings reset to defaults" }); }}>
          <RotateCcw className="w-4 h-4" /> Reset defaults
        </button>} />

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
        <Card title="Appearance">
          <div className="space-y-4">
            <div>
              <label className="vx-label">Theme</label>
              <div className="grid grid-cols-3 gap-2 mt-2">
                {([["light", Sun, "Light"], ["dark", Moon, "Dark"], ["system", Monitor, "System"]] as const).map(([val, Icon, label]) => (
                  <button key={val} onClick={() => updateSettings({ theme: val })}
                    className={`flex flex-col items-center gap-1.5 px-3 py-3 rounded-lg border transition ${settings.theme === val ? "bg-vx-600/15 border-vx-600/40 text-vx-500" : "border-[var(--vx-border)] hover:bg-vx-500/10"}`}>
                    <Icon className="w-4 h-4" />
                    <span className="text-xs">{label}</span>
                  </button>
                ))}
              </div>
            </div>
            <Toggle label="Animations" description="Subtle transitions and micro-interactions. Disable for reduced motion."
              checked={settings.animations} onChange={(v) => updateSettings({ animations: v })} />
            <div>
              <label className="vx-label">Language</label>
              <select className="vx-input mt-1" value={settings.language} onChange={(e) => updateSettings({ language: e.target.value })}>
                <option value="en">English</option>
              </select>
              <p className="text-[11px] text-[var(--vx-muted)] mt-1">The language architecture supports adding more locales; English is currently bundled.</p>
            </div>
          </div>
        </Card>

        <Card title="Monitoring & notifications">
          <div className="space-y-4">
            <div>
              <label className="vx-label">Monitoring interval</label>
              <select className="vx-input mt-1" value={settings.monitoring_interval_ms}
                onChange={(e) => updateSettings({ monitoring_interval_ms: Number(e.target.value) })}>
                <option value={500}>500 ms (more responsive)</option>
                <option value={1000}>1 second (default)</option>
                <option value={2000}>2 seconds</option>
                <option value={5000}>5 seconds (lower overhead)</option>
              </select>
            </div>
            <Toggle label="Background monitoring" description="Continue sampling sensors while other pages are open."
              checked={settings.background_monitoring} onChange={(v) => updateSettings({ background_monitoring: v })} />
            <Toggle label="Notifications" description="Show toast notifications for test results and alerts."
              checked={settings.notifications} onChange={(v) => updateSettings({ notifications: v })} />
          </div>
        </Card>

        <Card title="Reports & privacy">
          <div className="space-y-4">
            <div>
              <label className="vx-label">Default report format</label>
              <select className="vx-input mt-1" value={settings.report_default_format}
                onChange={(e) => updateSettings({ report_default_format: e.target.value as typeof settings.report_default_format })}>
                <option value="pdf">PDF</option>
                <option value="html">HTML</option>
                <option value="csv">CSV</option>
                <option value="json">JSON</option>
              </select>
            </div>
            <Toggle label="Include sensitive identifiers in reports" description="Serial numbers, MAC and IP addresses. Off by default; enable only when you need them."
              checked={settings.include_sensitive_in_reports} onChange={(v) => updateSettings({ include_sensitive_in_reports: v })} />
            <div className="rounded-lg bg-emerald-500/5 border border-emerald-500/20 p-3 text-xs text-[var(--vx-muted)] leading-relaxed">
              VYRONIX is offline-first. It sends no telemetry, no advertising trackers and no unauthorized data. All
              diagnostics and reports stay on this device unless you explicitly export them.
            </div>
          </div>
        </Card>

        <Card title="Startup & storage">
          <div className="space-y-4">
            <Toggle label="Start minimized" description="Launch to the system tray instead of the main window (desktop app)."
              checked={settings.start_minimized} onChange={(v) => updateSettings({ start_minimized: v })} />
            <div>
              <label className="vx-label">Local storage</label>
              <p className="text-xs text-[var(--vx-muted)] mt-1 leading-relaxed">
                Settings, inspection records and collected history are stored locally in an SQLite database in your
                application-data folder. {isNative ? "The desktop app manages this automatically." : "In this browser preview, settings use local storage."}
              </p>
            </div>
            <div>
              <label className="vx-label">Keyboard shortcuts</label>
              <div className="mt-1 space-y-1 text-xs text-[var(--vx-muted)]">
                <div className="flex justify-between"><span>Global search</span><kbd className="px-1.5 py-0.5 rounded bg-[var(--vx-border)]">Ctrl K</kbd></div>
                <div className="flex justify-between"><span>Dashboard</span><kbd className="px-1.5 py-0.5 rounded bg-[var(--vx-border)]">Ctrl 1</kbd></div>
                <div className="flex justify-between"><span>Reports</span><kbd className="px-1.5 py-0.5 rounded bg-[var(--vx-border)]">Ctrl R</kbd></div>
                <div className="flex justify-between"><span>Settings</span><kbd className="px-1.5 py-0.5 rounded bg-[var(--vx-border)]">Ctrl ,</kbd></div>
              </div>
            </div>
          </div>
        </Card>
      </div>
    </div>
  );
}

function Toggle({ label, description, checked, onChange }: { label: string; description: string; checked: boolean; onChange: (v: boolean) => void }) {
  return (
    <label className="flex items-start justify-between gap-3 cursor-pointer">
      <div>
        <div className="text-sm font-medium">{label}</div>
        <div className="text-[11px] text-[var(--vx-muted)] mt-0.5">{description}</div>
      </div>
      <button type="button" role="switch" aria-checked={checked} onClick={() => onChange(!checked)}
        className={`relative w-10 h-6 rounded-full transition shrink-0 ${checked ? "bg-vx-600" : "bg-[var(--vx-border)]"}`}>
        <span className={`absolute top-0.5 w-5 h-5 rounded-full bg-white transition-all ${checked ? "left-[18px]" : "left-0.5"}`} />
      </button>
    </label>
  );
}
