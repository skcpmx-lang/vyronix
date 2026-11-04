// Offline PDF report generation from the report model using jsPDF + autotable.
// No network access; produces a professional, branded, print-friendly document.

import { jsPDF } from "jspdf";
import autoTable from "jspdf-autotable";
import type { ReportModel } from "./types";
import { SEVERITY_LABELS } from "./types";

const BRAND = [27, 113, 245] as const; // vx-600
const INK = [17, 24, 39] as const;
const MUTED = [107, 114, 128] as const;

function arrayBufferToBase64(buf: ArrayBuffer): string {
  const bytes = new Uint8Array(buf);
  let binary = "";
  const chunk = 0x8000;
  for (let i = 0; i < bytes.length; i += chunk) {
    binary += String.fromCharCode(...bytes.subarray(i, i + chunk));
  }
  return btoa(binary);
}

export function generateReportPdf(model: ReportModel): string {
  const doc = new jsPDF({ unit: "pt", format: "a4" });
  const pageW = doc.internal.pageSize.getWidth();
  const margin = 40;
  let y = margin;

  // Header band
  doc.setFillColor(BRAND[0], BRAND[1], BRAND[2]);
  doc.rect(0, 0, pageW, 6, "F");
  doc.setFont("helvetica", "bold");
  doc.setFontSize(9);
  doc.setTextColor(BRAND[0], BRAND[1], BRAND[2]);
  doc.text("MERQO  ·  VYRONIX", margin, y + 8);
  y += 22;
  doc.setFontSize(20);
  doc.setTextColor(INK[0], INK[1], INK[2]);
  doc.text(model.title, margin, y + 8);
  y += 22;
  doc.setFont("helvetica", "normal");
  doc.setFontSize(9);
  doc.setTextColor(MUTED[0], MUTED[1], MUTED[2]);
  const when = new Date(model.generated_at_ms).toUTCString();
  doc.text(`Generated ${when}  ·  VYRONIX v${model.app_version}  ·  ${model.device.device_name}`, margin, y + 6);
  y += 24;

  // Device summary
  autoTable(doc, {
    startY: y,
    head: [["Device", ""]],
    body: [
      ["Device name", model.device.device_name],
      ["Manufacturer", model.device.manufacturer],
      ["Model", model.device.model],
      ["Operating system", model.device.os],
    ],
    theme: "grid",
    headStyles: { fillColor: [BRAND[0], BRAND[1], BRAND[2]], fontSize: 9 },
    styles: { fontSize: 9, cellPadding: 4 },
    columnStyles: { 0: { fontStyle: "bold", cellWidth: 130 } },
    margin: { left: margin, right: margin },
  });
  y = (doc as unknown as { lastAutoTable: { finalY: number } }).lastAutoTable.finalY + 18;

  // Overall score
  if (model.overall) {
    doc.setFont("helvetica", "bold");
    doc.setFontSize(12);
    doc.setTextColor(INK[0], INK[1], INK[2]);
    doc.text("Overall device condition", margin, y);
    y += 14;
    doc.setFont("helvetica", "normal");
    doc.setFontSize(10);
    if (model.overall.score != null) {
      doc.text(`Score: ${Math.round(model.overall.score)} / 100   ·   Coverage ${Math.round(model.overall.coverage * 100)}%   ·   Confidence: ${model.overall.confidence}`, margin, y);
    } else {
      doc.text("Insufficient measurable data for an overall score.", margin, y);
    }
    y += 12;
    autoTable(doc, {
      startY: y,
      head: [["Component", "Score", "Status", "Rationale"]],
      body: model.overall.components.map((c) => [
        c.label, c.score != null ? String(Math.round(c.score)) : "—",
        c.severity, c.rationale,
      ]),
      theme: "striped",
      headStyles: { fillColor: [BRAND[0], BRAND[1], BRAND[2]], fontSize: 8 },
      styles: { fontSize: 8, cellPadding: 3 },
      margin: { left: margin, right: margin },
    });
    y = (doc as unknown as { lastAutoTable: { finalY: number } }).lastAutoTable.finalY + 18;
  }

  // Sections
  for (const section of model.sections) {
    doc.setFont("helvetica", "bold");
    doc.setFontSize(12);
    doc.setTextColor(INK[0], INK[1], INK[2]);
    doc.text(section.title, margin, y);
    y += 8;
    autoTable(doc, {
      startY: y,
      head: [["Item", "Value", "Status", "Source"]],
      body: section.entries.map((e) => [e.label, e.value, e.status, e.source]),
      theme: "striped",
      headStyles: { fillColor: [BRAND[0], BRAND[1], BRAND[2]], fontSize: 8 },
      styles: { fontSize: 8, cellPadding: 3 },
      margin: { left: margin, right: margin },
    });
    y = (doc as unknown as { lastAutoTable: { finalY: number } }).lastAutoTable.finalY + 18;
  }

  // Findings
  if (model.findings.length > 0) {
    doc.setFont("helvetica", "bold");
    doc.setFontSize(12);
    doc.text("Diagnostic findings", margin, y);
    y += 8;
    autoTable(doc, {
      startY: y,
      head: [["Finding", "Severity", "Evidence", "Recommendation"]],
      body: model.findings.map((f) => [f.finding, SEVERITY_LABELS[f.severity], f.evidence, f.recommendation]),
      theme: "striped",
      headStyles: { fillColor: [BRAND[0], BRAND[1], BRAND[2]], fontSize: 8 },
      styles: { fontSize: 8, cellPadding: 3 },
      margin: { left: margin, right: margin },
    });
    y = (doc as unknown as { lastAutoTable: { finalY: number } }).lastAutoTable.finalY + 18;
  }

  // Recommendations
  if (model.recommendations.length > 0) {
    doc.setFont("helvetica", "bold");
    doc.setFontSize(12);
    doc.text("Usage recommendations", margin, y);
    y += 8;
    autoTable(doc, {
      startY: y,
      head: [["Use case", "Suitability", "Rationale"]],
      body: model.recommendations.map((r) => [r.use_case, r.suitability.replace(/_/g, " "), r.rationale]),
      theme: "striped",
      headStyles: { fillColor: [BRAND[0], BRAND[1], BRAND[2]], fontSize: 8 },
      styles: { fontSize: 8, cellPadding: 3 },
      margin: { left: margin, right: margin },
    });
  }

  // Footer on every page
  const pageCount = doc.getNumberOfPages();
  for (let i = 1; i <= pageCount; i++) {
    doc.setPage(i);
    const h = doc.internal.pageSize.getHeight();
    doc.setFont("helvetica", "normal");
    doc.setFontSize(7);
    doc.setTextColor(MUTED[0], MUTED[1], MUTED[2]);
    doc.text(
      "VYRONIX · A MERQO Software Product · Founder: Shohan Khan · merqoonline@gmail.com",
      margin, h - 24
    );
    doc.text(`Page ${i} of ${pageCount}`, pageW - margin, h - 24, { align: "right" });
  }

  return arrayBufferToBase64(doc.output("arraybuffer"));
}
