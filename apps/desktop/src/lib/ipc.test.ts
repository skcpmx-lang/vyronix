import { describe, expect, it } from "vitest";
import { unavailable, measurement, isNative, NativeUnavailableError, invoke } from "./ipc";

describe("honest measurement constructors", () => {
  it("unavailable carries no value and none confidence", () => {
    const m = unavailable<number>("test");
    expect(m.value).toBeNull();
    expect(m.status).toBe("unavailable");
    expect(m.confidence).toBe("none");
    expect(m.note).toBeTruthy();
  });
  it("available measurement has high confidence", () => {
    const m = measurement(42, "available", "sysinfo");
    expect(m.value).toBe(42);
    expect(m.confidence).toBe("high");
  });
  it("estimated measurement is not labelled available", () => {
    const m = measurement(42, "estimated", "derived");
    expect(m.status).toBe("estimated");
    expect(m.status).not.toBe("available");
  });
});

describe("browser preview honesty", () => {
  it("detects non-native environment in jsdom", () => {
    expect(isNative).toBe(false);
  });
  it("invoke throws NativeUnavailableError rather than fabricating data", async () => {
    await expect(invoke("get_cpu_info")).rejects.toBeInstanceOf(NativeUnavailableError);
  });
});
