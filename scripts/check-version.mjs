import { execSync } from "child_process";
import { readFile } from "fs/promises";

const cargoToml = await readFile("laplogos_core/Cargo.toml", "utf8");
const cargoVersionMatch = cargoToml.match(/^\s*version\s*=\s*"([^"]+)"/m);
if (!cargoVersionMatch) {
  throw new Error("Cargo.toml version not found");
}
const coreVersion = cargoVersionMatch[1];

const webPkg = JSON.parse(await readFile("laplogos_web/package.json", "utf8"));
const webVersion = webPkg.version;

if (coreVersion !== webVersion) {
  throw new Error(`version mismatch: core=${coreVersion} web=${webVersion}`);
}

const envTag =
  process.env.GITHUB_REF_TYPE === "tag" ? process.env.GITHUB_REF_NAME : undefined;
let tag = envTag;

if (!tag) {
  try {
    tag = execSync("git describe --tags --exact-match", { encoding: "utf8" }).trim();
  } catch {
    tag = "";
  }
}

if (tag.startsWith("v")) {
  const tagVersion = tag.slice(1);
  if (tagVersion !== coreVersion) {
    throw new Error(`tag mismatch: tag=${tagVersion} version=${coreVersion}`);
  }
}
