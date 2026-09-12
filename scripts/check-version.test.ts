import fs from "node:fs";
import os from "node:os";
import path from "node:path";
import { afterEach, describe, expect, it } from "vitest";
import { validateReleaseVersion } from "./check-version";

const temporaryRoots: string[] = [];

function fixture(versions: { package: string; cargo: string; tauri: string }): string {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), "cdp-version-contract-"));
  temporaryRoots.push(root);
  fs.mkdirSync(path.join(root, "src-tauri"));
  fs.writeFileSync(path.join(root, "package.json"), JSON.stringify({ version: versions.package }));
  fs.writeFileSync(
    path.join(root, "src-tauri", "Cargo.toml"),
    `[package]\nname = "fixture"\nversion = "${versions.cargo}"\n`,
  );
  fs.writeFileSync(
    path.join(root, "src-tauri", "tauri.conf.json"),
    JSON.stringify({ version: versions.tauri }),
  );
  return root;
}

afterEach(() => {
  for (const root of temporaryRoots.splice(0)) fs.rmSync(root, { recursive: true, force: true });
});

describe("release version contract", () => {
  it("accepts a synchronized stable release version", () => {
    expect(
      validateReleaseVersion(fixture({ package: "1.2.3", cargo: "1.2.3", tauri: "1.2.3" })),
    ).toBe("1.2.3");
  });

  it("rejects mismatched version sources", () => {
    expect(() =>
      validateReleaseVersion(fixture({ package: "1.2.3", cargo: "1.2.4", tauri: "1.2.3" })),
    ).toThrow("versions must match");
  });

  it("rejects the development placeholder and non-release SemVer", () => {
    expect(() =>
      validateReleaseVersion(fixture({ package: "0.0.0", cargo: "0.0.0", tauri: "0.0.0" })),
    ).toThrow("development placeholder");
    expect(() =>
      validateReleaseVersion(
        fixture({ package: "1.2.3-rc.1", cargo: "1.2.3-rc.1", tauri: "1.2.3-rc.1" }),
      ),
    ).toThrow("stable MAJOR.MINOR.PATCH");
  });
});
