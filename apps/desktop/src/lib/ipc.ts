// Low-level IPC bridge to the Tauri native backend.
//
// In a plain browser (dev preview) there is NO native backend. Rather than
// fabricate hardware data, every call returns an honest "unavailable" result
// and the UI shows a clear "desktop runtime required" notice. Mock data is
// never used in production.

import type { Measurement, Status } from "./types";

declare global {
  interface Window {
    __TAURI_INTERNALS__?: unknown;
  }
}

export const isNative = typeof window !== "undefined" && !!window.__TAURI_INTERNALS__;

export const NATIVE_REQUIRED_NOTE =
  "Hardware access requires the VYRONIX Windows desktop application. This browser preview cannot read real hardware.";

type InvokeFn = <T>(cmd: string, args?: Record<string, unknown>) => Promise<T>;

let invokeImpl: InvokeFn | null = null;

async function getInvoke(): Promise<InvokeFn | null> {
  if (!isNative) return null;
  if (!invokeImpl) {
    const mod = await import("@tauri-apps/api/core");
    invokeImpl = mod.invoke as InvokeFn;
  }
  return invokeImpl;
}

/// Invoke a native command. Throws in native mode on error; in browser mode
/// throws a NativeUnavailableError so callers can render an honest state.
export class NativeUnavailableError extends Error {
  constructor() {
    super(NATIVE_REQUIRED_NOTE);
    this.name = "NativeUnavailableError";
  }
}

export async function invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  const inv = await getInvoke();
  if (!inv) throw new NativeUnavailableError();
  return inv<T>(cmd, args);
}

/// Listen to a native event. Returns an unlisten function. No-op in browser.
export async function listen<T>(
  event: string,
  handler: (payload: T) => void
): Promise<() => void> {
  if (!isNative) return () => {};
  const mod = await import("@tauri-apps/api/event");
  return mod.listen<T>(event, (e) => handler(e.payload));
}

// ---- Honest measurement constructors for unavailable states ----

export function unavailable<T>(source: string, note = NATIVE_REQUIRED_NOTE): Measurement<T> {
  return {
    value: null,
    status: "unavailable",
    confidence: "none",
    source,
    note,
    timestamp_ms: 0,
  };
}

export function measurement<T>(
  value: T | null,
  status: Status,
  source: string,
  note?: string
): Measurement<T> {
  return {
    value,
    status,
    confidence: status === "available" || status === "verified" ? "high" : "low",
    source,
    note: note ?? null,
    timestamp_ms: Date.now(),
  };
}
