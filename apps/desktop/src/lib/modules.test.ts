import { describe, expect, it } from "vitest";
import { MODULES, buildSearchIndex, searchIndex, moduleById } from "./modules";

describe("module registry", () => {
  it("has unique ids", () => {
    const ids = MODULES.map((m) => m.id);
    expect(new Set(ids).size).toBe(ids.length);
  });
  it("resolves modules by id", () => {
    expect(moduleById("battery")?.title).toBe("Battery");
    expect(moduleById("welcome")?.title).toBe("Welcome");
  });
});

describe("searchIndex", () => {
  it("returns empty for empty query", () => {
    expect(searchIndex("")).toEqual([]);
    expect(searchIndex("   ")).toEqual([]);
  });
  it("finds battery by keyword", () => {
    const results = searchIndex("battery");
    expect(results.length).toBeGreaterThan(0);
    expect(results[0].page).toBe("battery");
  });
  it("finds settings entries", () => {
    const results = searchIndex("theme");
    expect(results.some((r) => r.kind === "setting")).toBe(true);
  });
  it("ranks title matches above description matches", () => {
    const results = searchIndex("storage");
    expect(results[0].title.toLowerCase()).toContain("storage");
  });
  it("index covers all modules", () => {
    const index = buildSearchIndex();
    const moduleEntries = index.filter((e) => e.kind === "module");
    expect(moduleEntries.length).toBe(MODULES.length);
  });
});
