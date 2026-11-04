import { describe, expect, it } from "vitest";
import {
  formatBytes, formatPct, formatDuration, formatFreq, formatMwh, formatTemp, clamp, formatHours,
} from "./format";

describe("formatBytes", () => {
  it("formats zero and null honestly", () => {
    expect(formatBytes(0)).toBe("0 B");
    expect(formatBytes(null)).toBe("—");
    expect(formatBytes(undefined)).toBe("—");
  });
  it("formats binary units", () => {
    expect(formatBytes(1024)).toBe("1.0 KB");
    expect(formatBytes(1024 * 1024)).toBe("1.0 MB");
    expect(formatBytes(16 * 1024 * 1024 * 1024)).toBe("16.0 GB");
  });
});

describe("formatPct", () => {
  it("shows em-dash for unknown, never fabricates", () => {
    expect(formatPct(null)).toBe("—");
    expect(formatPct(87.4)).toBe("87%");
    expect(formatPct(87.4, 1)).toBe("87.4%");
  });
});

describe("formatDuration", () => {
  it("formats compound durations", () => {
    expect(formatDuration(45)).toBe("45s");
    expect(formatDuration(125)).toBe("2m 5s");
    expect(formatDuration(3661)).toBe("1h 1m 1s");
    expect(formatDuration(90061)).toBe("1d 1h 1m");
    expect(formatDuration(null)).toBe("—");
  });
});

describe("formatFreq", () => {
  it("switches to GHz", () => {
    expect(formatFreq(800)).toBe("800 MHz");
    expect(formatFreq(3600)).toBe("3.60 GHz");
    expect(formatFreq(null)).toBe("—");
  });
});

describe("formatMwh / formatTemp / formatHours", () => {
  it("formats battery units", () => {
    expect(formatMwh(50000)).toBe("50.00 Wh");
    expect(formatMwh(500)).toBe("500 mWh");
    expect(formatTemp(45)).toBe("45 °C");
    expect(formatTemp(null)).toBe("—");
    expect(formatHours(2.5)).toBe("2h 30m");
  });
});

describe("clamp", () => {
  it("bounds values", () => {
    expect(clamp(5, 0, 10)).toBe(5);
    expect(clamp(-5, 0, 10)).toBe(0);
    expect(clamp(15, 0, 10)).toBe(10);
  });
});
