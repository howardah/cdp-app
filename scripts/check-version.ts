import fs from "node:fs";
import path from "node:path";

const STABLE_SEMVER = /^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)$/;

function readJsonVersion(filePath: string): string {
  const value: unknown = JSON.parse(fs.readFileSync(filePath, "utf8"));
  if (
    typeof value !== "object" ||
    value === null ||
    !("version" in value) ||
    typeof value.version !== "string"
  ) {
    throw new Error(`${filePath} must contain a string version`);
  }
  return value.version;
}

function readCargoPackageVersion(filePath: string): string {
  let inPackage = false;
  for (const line of fs.readFileSync(filePath, "utf8").split(/\r?\n/)) {
    const section = line.trim();
    if (section.startsWith("[") && section.endsWith("]")) {
      inPackage = section === "[package]";
      continue;
    }
    if (!inPackage) continue;
    const match = line.match(/^\s*version\s*=\s*"([^"]+)"\s*(?:#.*)?$/);
    if (match) return match[1];
  }
  throw new Error(`${filePath} must contain [package].version`);
}

/**
 * Enforce the single version source contract used by release automation.
 * Returns the verified version so CI can use it for an artifact/tag check.
 */
export function validateReleaseVersion(root = process.cwd()): string {
  const packageVersion = readJsonVersion(path.join(root, "package.json"));
  const cargoVersion = readCargoPackageVersion(path.join(root, "src-tauri", "Cargo.toml"));
  const tauriVersion = readJsonVersion(path.join(root, "src-tauri", "tauri.conf.json"));
  const versions = [packageVersion, cargoVersion, tauriVersion];

  if (new Set(versions).size !== 1) {
    throw new Error(
      `versions must match: package.json=${packageVersion}, Cargo.toml=${cargoVersion}, tauri.conf.json=${tauriVersion}`,
    );
  }
  if (!STABLE_SEMVER.test(packageVersion)) {
    throw new Error(`version must use stable MAJOR.MINOR.PATCH SemVer, got ${packageVersion}`);
  }
  if (packageVersion === "0.0.0") {
    throw new Error("version 0.0.0 is a development placeholder and cannot be released");
  }
  return packageVersion;
}

if (import.meta.main) {
  try {
    console.log(validateReleaseVersion());
  } catch (error) {
    console.error(error instanceof Error ? `Version contract failed: ${error.message}` : error);
    process.exitCode = 1;
  }
}
