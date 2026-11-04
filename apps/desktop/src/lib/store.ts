// Global application state (zustand): settings, theme, navigation, toasts.

import { create } from "zustand";
import type { AppSettings } from "./types";
import { DEFAULT_SETTINGS } from "./types";
import { getSettings, saveSettings } from "./services";
import { isNative } from "./ipc";

export type PageId =
  | "welcome" | "dashboard" | "system" | "cpu" | "gpu" | "memory" | "battery"
  | "storage" | "monitoring" | "display" | "keyboard" | "mouse" | "audio"
  | "benchmarks" | "stress" | "windows" | "network" | "inspection" | "reports"
  | "settings" | "about";

export interface Toast {
  id: number;
  kind: "info" | "success" | "warning" | "error";
  title: string;
  message?: string;
}

interface AppState {
  page: PageId;
  setPage: (p: PageId) => void;

  settings: AppSettings;
  settingsLoaded: boolean;
  loadSettings: () => Promise<void>;
  updateSettings: (patch: Partial<AppSettings>) => Promise<void>;
  resetSettings: () => Promise<void>;

  resolvedTheme: "dark" | "light";
  applyTheme: () => void;

  toasts: Toast[];
  pushToast: (t: Omit<Toast, "id">) => void;
  dismissToast: (id: number) => void;

  searchOpen: boolean;
  setSearchOpen: (open: boolean) => void;

  welcomeSeen: boolean;
  setWelcomeSeen: (seen: boolean) => void;
}

let toastId = 0;

function systemPrefersDark(): boolean {
  if (typeof window === "undefined" || !window.matchMedia) return true;
  return window.matchMedia("(prefers-color-scheme: dark)").matches;
}

function resolveTheme(theme: AppSettings["theme"]): "dark" | "light" {
  if (theme === "system") return systemPrefersDark() ? "dark" : "light";
  return theme;
}

export const useAppStore = create<AppState>((set, get) => ({
  page: "welcome",
  setPage: (p) => set({ page: p }),

  settings: DEFAULT_SETTINGS,
  settingsLoaded: false,
  loadSettings: async () => {
    try {
      const settings = await getSettings();
      set({ settings, settingsLoaded: true });
    } catch {
      set({ settings: DEFAULT_SETTINGS, settingsLoaded: true });
    }
    get().applyTheme();
  },
  updateSettings: async (patch) => {
    const next = { ...get().settings, ...patch };
    set({ settings: next });
    get().applyTheme();
    try {
      await saveSettings(next);
    } catch {
      get().pushToast({ kind: "error", title: "Could not save settings" });
    }
  },
  resetSettings: async () => {
    set({ settings: DEFAULT_SETTINGS });
    get().applyTheme();
    try {
      await saveSettings(DEFAULT_SETTINGS);
    } catch { /* ignore */ }
  },

  resolvedTheme: "dark",
  applyTheme: () => {
    const theme = resolveTheme(get().settings.theme);
    set({ resolvedTheme: theme });
    if (typeof document !== "undefined") {
      const root = document.documentElement;
      root.classList.toggle("dark", theme === "dark");
      root.style.colorScheme = theme;
    }
  },

  toasts: [],
  pushToast: (t) => {
    const id = ++toastId;
    set((s) => ({ toasts: [...s.toasts, { ...t, id }] }));
    if (get().settings.notifications) {
      setTimeout(() => get().dismissToast(id), 6000);
    }
  },
  dismissToast: (id) => set((s) => ({ toasts: s.toasts.filter((t) => t.id !== id) })),

  searchOpen: false,
  setSearchOpen: (open) => set({ searchOpen: open }),

  welcomeSeen: false,
  setWelcomeSeen: (seen) => {
    set({ welcomeSeen: seen });
    try { localStorage.setItem("vyronix.welcomeSeen", seen ? "1" : "0"); } catch { /* ignore */ }
  },
}));

// Initialize welcomeSeen from localStorage (browser preview convenience).
if (typeof localStorage !== "undefined") {
  try {
    if (localStorage.getItem("vyronix.welcomeSeen") === "1") {
      useAppStore.setState({ welcomeSeen: true, page: "dashboard" });
    }
  } catch { /* ignore */ }
}

export { isNative };
