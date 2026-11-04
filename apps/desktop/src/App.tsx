import { useEffect } from "react";
import { useAppStore } from "./lib/store";
import { Sidebar } from "./components/Sidebar";
import { Topbar } from "./components/Topbar";
import { Toasts } from "./components/Toasts";
import { SearchPalette } from "./components/SearchPalette";

import { Welcome } from "./pages/Welcome";
import { Dashboard } from "./pages/Dashboard";
import { SystemOverview } from "./pages/SystemOverview";
import { CpuPage } from "./pages/CpuPage";
import { GpuPage } from "./pages/GpuPage";
import { MemoryPage } from "./pages/MemoryPage";
import { BatteryPage } from "./pages/BatteryPage";
import { StoragePage } from "./pages/StoragePage";
import { MonitoringPage } from "./pages/MonitoringPage";
import { DisplayTest } from "./pages/DisplayTest";
import { KeyboardTest } from "./pages/KeyboardTest";
import { MouseTest } from "./pages/MouseTest";
import { AudioTest } from "./pages/AudioTest";
import { Benchmarks } from "./pages/Benchmarks";
import { StressTest } from "./pages/StressTest";
import { WindowsDiagnostics } from "./pages/WindowsDiagnostics";
import { NetworkPage } from "./pages/NetworkPage";
import { Inspection } from "./pages/Inspection";
import { Reports } from "./pages/Reports";
import { SettingsPage } from "./pages/SettingsPage";
import { About } from "./pages/About";

export default function App() {
  const page = useAppStore((s) => s.page);
  const settings = useAppStore((s) => s.settings);
  const setSearchOpen = useAppStore((s) => s.setSearchOpen);
  const setPage = useAppStore((s) => s.setPage);

  // Global keyboard shortcuts.
  useEffect(() => {
    const onKey = (e: KeyboardEvent) => {
      const mod = e.ctrlKey || e.metaKey;
      if (mod && e.key.toLowerCase() === "k") {
        e.preventDefault();
        setSearchOpen(true);
      } else if (mod && e.key === "1") {
        e.preventDefault();
        setPage("dashboard");
      } else if (mod && e.key.toLowerCase() === "r") {
        e.preventDefault();
        setPage("reports");
      } else if (mod && e.key === ",") {
        e.preventDefault();
        setPage("settings");
      }
    };
    window.addEventListener("keydown", onKey);
    return () => window.removeEventListener("keydown", onKey);
  }, [setSearchOpen, setPage]);

  // Honor the animations toggle.
  useEffect(() => {
    document.documentElement.classList.toggle("no-animations", !settings.animations);
  }, [settings.animations]);

  return (
    <div className="flex h-full overflow-hidden">
      {page !== "welcome" && <Sidebar />}
      <div className="flex-1 flex flex-col min-w-0">
        {page !== "welcome" && <Topbar />}
        <main className="flex-1 overflow-y-auto">
          <div className={page === "welcome" ? "" : "p-6 max-w-6xl mx-auto w-full"}>
            <PageSwitch />
          </div>
        </main>
      </div>
      <Toasts />
      <SearchPalette />
    </div>
  );
}

function PageSwitch() {
  const page = useAppStore((s) => s.page);
  switch (page) {
    case "welcome": return <Welcome />;
    case "dashboard": return <Dashboard />;
    case "system": return <SystemOverview />;
    case "cpu": return <CpuPage />;
    case "gpu": return <GpuPage />;
    case "memory": return <MemoryPage />;
    case "battery": return <BatteryPage />;
    case "storage": return <StoragePage />;
    case "monitoring": return <MonitoringPage />;
    case "display": return <DisplayTest />;
    case "keyboard": return <KeyboardTest />;
    case "mouse": return <MouseTest />;
    case "audio": return <AudioTest />;
    case "benchmarks": return <Benchmarks />;
    case "stress": return <StressTest />;
    case "windows": return <WindowsDiagnostics />;
    case "network": return <NetworkPage />;
    case "inspection": return <Inspection />;
    case "reports": return <Reports />;
    case "settings": return <SettingsPage />;
    case "about": return <About />;
    default: return <Dashboard />;
  }
}
