import { describe, expect, it } from "vitest";
import { findRecipe, recipeCatalog, searchRecipes, validateRecipes } from "./index";

describe("recipe catalog", () => {
  it("contains eight unique, searchable guided recipes", () => {
    expect(recipeCatalog).toHaveLength(8);
    expect(new Set(recipeCatalog.map((recipe) => recipe.id)).size).toBe(8);
    expect(searchRecipes("spectral").map((recipe) => recipe.id)).toEqual([
      "spectral-round-trip",
      "smooth-and-exaggerate-spectrum",
    ]);
    expect(searchRecipes("mixfile").map((recipe) => recipe.id)).toEqual(["step-pan-render"]);
  });

  it("references compatible process modes", () => {
    expect(() => validateRecipes(recipeCatalog)).not.toThrow();
    expect(findRecipe("pitch-then-polish")?.steps.map((step) => step.modeId)).toEqual([
      "semitones",
      "normalise",
    ]);
  });

  it("rejects missing modes and incompatible handoffs", () => {
    expect(() =>
      validateRecipes([
        {
          ...recipeCatalog[0],
          id: "missing-mode",
          steps: [{ ...recipeCatalog[0].steps[0], modeId: "missing" }, recipeCatalog[0].steps[1]],
        },
      ]),
    ).toThrow("unknown recipe mode");
    expect(() =>
      validateRecipes([
        {
          ...recipeCatalog[0],
          id: "bad-handoff",
          steps: [recipeCatalog[0].steps[0], recipeCatalog[3].steps[1]],
        },
      ]),
    ).toThrow("incompatible recipe handoff");
  });
});
