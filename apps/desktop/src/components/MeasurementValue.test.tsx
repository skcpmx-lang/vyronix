import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/react";
import { MeasurementValue } from "./MeasurementValue";
import { StatusBadge, SeverityBadge } from "./StatusBadge";
import type { Measurement } from "../lib/types";

describe("MeasurementValue", () => {
  it("renders an available value with its status", () => {
    const m: Measurement<number> = { value: 90, status: "available", confidence: "high", source: "IOCTL", timestamp_ms: 0 };
    render(<MeasurementValue label="Health" measurement={m} format={(v) => `${v}%`} />);
    expect(screen.getByText("90%")).toBeInTheDocument();
    expect(screen.getByText("Available")).toBeInTheDocument();
  });

  it("renders em-dash (not a fabricated number) when unavailable", () => {
    const m: Measurement<number> = { value: null, status: "unavailable", confidence: "none", source: "x", timestamp_ms: 0 };
    render(<MeasurementValue label="Health" measurement={m} />);
    expect(screen.getByText("—")).toBeInTheDocument();
    expect(screen.getByText("Unavailable")).toBeInTheDocument();
  });

  it("labels estimated values distinctly", () => {
    const m: Measurement<number> = { value: 5, status: "estimated", confidence: "medium", source: "calc", note: "derived", timestamp_ms: 0 };
    render(<MeasurementValue label="Runtime" measurement={m} />);
    expect(screen.getByText("Estimated")).toBeInTheDocument();
  });
});

describe("badges", () => {
  it("shows Unknown distinctly from Good", () => {
    const { rerender } = render(<SeverityBadge severity="unknown" />);
    expect(screen.getByText("Unknown")).toBeInTheDocument();
    rerender(<SeverityBadge severity="good" />);
    expect(screen.getByText("Good")).toBeInTheDocument();
  });
  it("renders permission required status", () => {
    render(<StatusBadge status="permission_required" />);
    expect(screen.getByText("Permission Required")).toBeInTheDocument();
  });
});
