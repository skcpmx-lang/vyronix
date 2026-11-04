// Module registry: drives sidebar navigation and global search.

import {
  LayoutDashboard, Monitor, Cpu, CircuitBoard, MemoryStick, BatteryCharging,
  HardDrive, Activity, MonitorPlay, Keyboard, Mouse, AudioLines, Gauge, Flame,
  AppWindow, Network, ClipboardCheck, FileText, Settings, Info, Sparkles,
  type LucideIcon,
} from "lucide-react";
import type { PageId } from "./store";

export interface ModuleDef {
  id: PageId;
  title: string;
  description: string;
  icon: LucideIcon;
  category: "Overview" | "Hardware" | "Testing" | "Performance" | "System" | "Tools" | "Application";
  keywords: string[];
}

export const MODULES: ModuleDef[] = [
  { id: "dashboard", title: "Dashboard", description: "Device condition, live metrics and quick actions", icon: LayoutDashboard, category: "Overview", keywords: ["home", "overview", "score", "health"] },
  { id: "system", title: "System Overview", description: "Device, motherboard, BIOS and Windows identity", icon: Monitor, category: "Overview", keywords: ["device", "bios", "motherboard", "windows", "tpm", "secure boot"] },
  { id: "cpu", title: "Processor", description: "CPU specifications, usage and temperature", icon: Cpu, category: "Hardware", keywords: ["processor", "cores", "threads", "frequency", "ghz"] },
  { id: "gpu", title: "Graphics", description: "GPU adapters, drivers and memory", icon: CircuitBoard, category: "Hardware", keywords: ["graphics", "video", "vram", "display adapter"] },
  { id: "memory", title: "Memory", description: "RAM capacity, modules and testing", icon: MemoryStick, category: "Hardware", keywords: ["ram", "dimm", "ddr", "memory test"] },
  { id: "battery", title: "Battery", description: "Battery health, wear and charging diagnostics", icon: BatteryCharging, category: "Hardware", keywords: ["battery", "charge", "wear", "capacity", "cycle"] },
  { id: "storage", title: "Storage", description: "Drives, SMART health and partitions", icon: HardDrive, category: "Hardware", keywords: ["disk", "ssd", "hdd", "nvme", "smart"] },
  { id: "monitoring", title: "Live Monitoring", description: "Real-time hardware sensors and graphs", icon: Activity, category: "Hardware", keywords: ["sensors", "temperature", "usage", "graphs", "live"] },
  { id: "display", title: "Display Test", description: "Full-screen patterns for pixel and uniformity inspection", icon: MonitorPlay, category: "Testing", keywords: ["screen", "pixel", "dead pixel", "backlight", "color"] },
  { id: "keyboard", title: "Keyboard Test", description: "Interactive key press verification", icon: Keyboard, category: "Testing", keywords: ["keys", "typing", "stuck key"] },
  { id: "mouse", title: "Mouse & Touchpad", description: "Button, scroll and gesture verification", icon: Mouse, category: "Testing", keywords: ["click", "scroll", "touchpad", "pointer"] },
  { id: "audio", title: "Audio & Microphone", description: "Speaker and microphone testing", icon: AudioLines, category: "Testing", keywords: ["speaker", "sound", "mic", "microphone", "recording"] },
  { id: "benchmarks", title: "Benchmarks", description: "CPU, memory and storage performance tests", icon: Gauge, category: "Performance", keywords: ["speed", "performance", "score", "iops", "bandwidth"] },
  { id: "stress", title: "Stress Test", description: "Controlled CPU load with safety limits", icon: Flame, category: "Performance", keywords: ["load", "stability", "thermal", "throttle"] },
  { id: "windows", title: "Windows Diagnostics", description: "Drivers, services, startup and event logs", icon: AppWindow, category: "System", keywords: ["drivers", "services", "events", "bsod", "startup"] },
  { id: "network", title: "Network", description: "Adapters, connectivity, ping and DNS", icon: Network, category: "System", keywords: ["wifi", "ethernet", "ip", "dns", "ping", "bluetooth"] },
  { id: "inspection", title: "Device Inspection", description: "Guided full-device inspection checklist", icon: ClipboardCheck, category: "Tools", keywords: ["checklist", "inspection", "physical", "ports", "hinges"] },
  { id: "reports", title: "Reports", description: "Generate and export diagnostic reports", icon: FileText, category: "Tools", keywords: ["export", "pdf", "html", "csv", "json", "report"] },
  { id: "settings", title: "Settings", description: "Theme, monitoring, privacy and preferences", icon: Settings, category: "Application", keywords: ["theme", "dark", "light", "privacy", "preferences"] },
  { id: "about", title: "About", description: "Product, version and license information", icon: Info, category: "Application", keywords: ["version", "license", "credits", "merqo"] },
];

export const WELCOME_MODULE: ModuleDef = {
  id: "welcome", title: "Welcome", description: "Welcome to VYRONIX", icon: Sparkles,
  category: "Overview", keywords: ["welcome", "start"],
};

export function moduleById(id: PageId): ModuleDef | undefined {
  if (id === "welcome") return WELCOME_MODULE;
  return MODULES.find((m) => m.id === id);
}

export interface SearchEntry {
  id: string;
  title: string;
  description: string;
  page: PageId;
  kind: "module" | "setting" | "tool";
  keywords: string[];
}

export function buildSearchIndex(): SearchEntry[] {
  const entries: SearchEntry[] = MODULES.map((m) => ({
    id: `module:${m.id}`, title: m.title, description: m.description,
    page: m.id, kind: "module", keywords: m.keywords,
  }));
  const settings: [string, string, string[]][] = [
    ["Theme", "Switch between dark, light and system theme", ["dark", "light", "appearance"]],
    ["Monitoring interval", "Configure sensor polling frequency", ["poll", "interval", "sensors"]],
    ["Privacy", "Control sensitive identifiers in reports", ["privacy", "serial", "mac", "ip"]],
    ["Animations", "Reduce or disable interface animations", ["motion", "animation", "accessibility"]],
    ["Notifications", "Enable or disable toast notifications", ["toast", "alerts"]],
  ];
  for (const [title, description, keywords] of settings) {
    entries.push({ id: `setting:${title}`, title, description, page: "settings", kind: "setting", keywords });
  }
  return entries;
}

export function searchIndex(query: string): SearchEntry[] {
  const q = query.trim().toLowerCase();
  if (!q) return [];
  const index = buildSearchIndex();
  return index
    .map((e) => {
      let score = 0;
      const title = e.title.toLowerCase();
      const desc = e.description.toLowerCase();
      if (title === q) score += 100;
      else if (title.startsWith(q)) score += 60;
      else if (title.includes(q)) score += 40;
      if (desc.includes(q)) score += 15;
      if (e.keywords.some((k) => k.includes(q))) score += 25;
      return { e, score };
    })
    .filter((x) => x.score > 0)
    .sort((a, b) => b.score - a.score)
    .slice(0, 8)
    .map((x) => x.e);
}
