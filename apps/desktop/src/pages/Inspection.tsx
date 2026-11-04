import { useMemo, useState } from "react";
import { ClipboardCheck, Save } from "lucide-react";
import { Card, PageHeader } from "../components/Card";
import { useAppStore } from "../lib/store";
import { isNative } from "../lib/ipc";
import { getInspectionItems, setInspectionItem } from "../lib/services";
import { useAsync } from "../hooks/useAsync";
import { Spinner } from "../components/ProgressBar";
import type { InspectionItem, InspectionStatus } from "../lib/types";

const STATUS_OPTIONS: { value: InspectionStatus; label: string; color: string }[] = [
  { value: "not_tested", label: "Not Tested", color: "text-slate-400" },
  { value: "passed", label: "Passed", color: "text-emerald-500" },
  { value: "warning", label: "Warning", color: "text-amber-500" },
  { value: "failed", label: "Failed", color: "text-red-500" },
  { value: "unsupported", label: "Unsupported", color: "text-slate-400" },
  { value: "manual_verification_required", label: "Manual Verification", color: "text-violet-400" },
];

// Guided inspection categories. Physical items are manual-verification by
// default; a port is never marked working unless actually tested.
const DEFAULT_ITEMS: Omit<InspectionItem, "updated_at_ms">[] = [
  { id: "cpu", category: "Core hardware", name: "CPU", status: "not_tested", notes: "", automated: true },
  { id: "gpu", category: "Core hardware", name: "GPU", status: "not_tested", notes: "", automated: true },
  { id: "ram", category: "Core hardware", name: "RAM", status: "not_tested", notes: "", automated: true },
  { id: "ssd", category: "Storage", name: "SSD", status: "not_tested", notes: "", automated: true },
  { id: "hdd", category: "Storage", name: "HDD", status: "not_tested", notes: "", automated: true },
  { id: "battery", category: "Power", name: "Battery", status: "not_tested", notes: "", automated: true },
  { id: "display", category: "Display & input", name: "Display", status: "not_tested", notes: "", automated: false },
  { id: "keyboard", category: "Display & input", name: "Keyboard", status: "not_tested", notes: "", automated: false },
  { id: "touchpad", category: "Display & input", name: "Touchpad", status: "not_tested", notes: "", automated: false },
  { id: "mouse", category: "Display & input", name: "Mouse", status: "not_tested", notes: "", automated: false },
  { id: "speaker", category: "Audio", name: "Speaker", status: "not_tested", notes: "", automated: false },
  { id: "microphone", category: "Audio", name: "Microphone", status: "not_tested", notes: "", automated: false },
  { id: "camera", category: "Audio", name: "Camera", status: "not_tested", notes: "", automated: false },
  { id: "usb", category: "Ports & connectivity", name: "USB ports", status: "manual_verification_required", notes: "", automated: false },
  { id: "hdmi", category: "Ports & connectivity", name: "HDMI", status: "manual_verification_required", notes: "", automated: false },
  { id: "displayport", category: "Ports & connectivity", name: "DisplayPort", status: "manual_verification_required", notes: "", automated: false },
  { id: "audiojack", category: "Ports & connectivity", name: "Audio jack", status: "manual_verification_required", notes: "", automated: false },
  { id: "ethernet", category: "Ports & connectivity", name: "Ethernet", status: "not_tested", notes: "", automated: true },
  { id: "wifi", category: "Ports & connectivity", name: "Wi-Fi", status: "not_tested", notes: "", automated: true },
  { id: "bluetooth", category: "Ports & connectivity", name: "Bluetooth", status: "not_tested", notes: "", automated: true },
  { id: "cooling", category: "Cooling & power", name: "Cooling system", status: "manual_verification_required", notes: "", automated: false },
  { id: "fans", category: "Cooling & power", name: "Fans", status: "manual_verification_required", notes: "", automated: false },
  { id: "adapter", category: "Cooling & power", name: "Power adapter", status: "manual_verification_required", notes: "", automated: false },
  { id: "physical", category: "Physical condition", name: "Physical condition", status: "manual_verification_required", notes: "", automated: false },
  { id: "hinges", category: "Physical condition", name: "Hinges", status: "manual_verification_required", notes: "", automated: false },
  { id: "chassis", category: "Physical condition", name: "Chassis", status: "manual_verification_required", notes: "", automated: false },
  { id: "screws", category: "Physical condition", name: "Screws", status: "manual_verification_required", notes: "", automated: false },
  { id: "chargingport", category: "Physical condition", name: "Charging port", status: "manual_verification_required", notes: "", automated: false },
  { id: "other", category: "Physical condition", name: "Other components", status: "manual_verification_required", notes: "", automated: false },
];

export function Inspection() {
  const pushToast = useAppStore((s) => s.pushToast);
  const remote = useAsync(async () => (isNative ? getInspectionItems() : null), []);
  const [local, setLocal] = useState<InspectionItem[]>(
    DEFAULT_ITEMS.map((d) => ({ ...d, updated_at_ms: 0 }))
  );

  const items = remote.data && remote.data.length > 0 ? remote.data : local;

  const update = (id: string, patch: Partial<InspectionItem>) => {
    const next = items.map((it) => it.id === id ? { ...it, ...patch, updated_at_ms: Date.now() } : it);
    setLocal(next);
    const changed = next.find((it) => it.id === id);
    if (isNative && changed) {
      setInspectionItem(changed).catch(() => pushToast({ kind: "error", title: "Could not save inspection item" }));
    }
  };

  const categories = useMemo(() => {
    const map = new Map<string, InspectionItem[]>();
    for (const it of items) {
      if (!map.has(it.category)) map.set(it.category, []);
      map.get(it.category)!.push(it);
    }
    return Array.from(map.entries());
  }, [items]);

  const counts = useMemo(() => {
    const c = { passed: 0, warning: 0, failed: 0, not_tested: 0, other: 0 };
    for (const it of items) {
      if (it.status === "passed") c.passed++;
      else if (it.status === "warning") c.warning++;
      else if (it.status === "failed") c.failed++;
      else if (it.status === "not_tested") c.not_tested++;
      else c.other++;
    }
    return c;
  }, [items]);

  return (
    <div>
      <PageHeader title="Device Inspection" description="Guided full-device inspection — physical items require hands-on verification"
        actions={isNative ? <button className="vx-btn-ghost" onClick={() => pushToast({ kind: "success", title: "Inspection saved", message: "Changes are stored locally as you edit." })}><Save className="w-4 h-4" /> Saved locally</button> : undefined} />

      <div className="grid grid-cols-2 md:grid-cols-5 gap-3 mb-4">
        <Summary label="Passed" value={counts.passed} color="text-emerald-500" />
        <Summary label="Warning" value={counts.warning} color="text-amber-500" />
        <Summary label="Failed" value={counts.failed} color="text-red-500" />
        <Summary label="Not tested" value={counts.not_tested} color="text-slate-400" />
        <Summary label="Manual / unsupported" value={counts.other} color="text-violet-400" />
      </div>

      {remote.loading ? <Spinner /> : (
        <div className="space-y-4">
          {categories.map(([cat, catItems]) => (
            <Card key={cat} title={cat}>
              <div className="space-y-2">
                {catItems.map((it) => (
                  <div key={it.id} className="rounded-lg border border-[var(--vx-border)] p-3">
                    <div className="flex flex-wrap items-center gap-3">
                      <div className="min-w-[140px]">
                        <div className="text-sm font-medium">{it.name}</div>
                        <div className="text-[10px] text-[var(--vx-muted)]">{it.automated ? "Can be automated" : "Manual verification"}</div>
                      </div>
                      <select
                        className={`vx-input w-auto text-sm ${STATUS_OPTIONS.find((s) => s.value === it.status)?.color ?? ""}`}
                        value={it.status}
                        onChange={(e) => update(it.id, { status: e.target.value as InspectionStatus })}
                        aria-label={`Status for ${it.name}`}
                      >
                        {STATUS_OPTIONS.map((s) => <option key={s.value} value={s.value}>{s.label}</option>)}
                      </select>
                      <input
                        className="vx-input flex-1 min-w-[180px]"
                        placeholder="Notes / evidence…"
                        value={it.notes}
                        onChange={(e) => update(it.id, { notes: e.target.value })}
                        aria-label={`Notes for ${it.name}`}
                      />
                    </div>
                  </div>
                ))}
              </div>
            </Card>
          ))}
          <p className="text-xs text-[var(--vx-muted)] flex items-start gap-2">
            <ClipboardCheck className="w-4 h-4 shrink-0 mt-0.5" />
            A physical port or component is never marked “Passed” unless it has actually been tested. Use the dedicated test
            modules for automatable items, and hands-on verification for physical ones. The inspection report captures every
            item’s status, notes and timestamp.
          </p>
        </div>
      )}
    </div>
  );
}

function Summary({ label, value, color }: { label: string; value: number; color: string }) {
  return (
    <div className="vx-panel p-3 text-center">
      <div className={`text-2xl font-bold tabular-nums ${color}`}>{value}</div>
      <div className="text-[11px] text-[var(--vx-muted)]">{label}</div>
    </div>
  );
}
