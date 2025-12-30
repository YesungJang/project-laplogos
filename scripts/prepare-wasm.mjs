import { copyFile, mkdir } from "fs/promises";
import { resolve } from "path";

const source = resolve("laplogos_core/pkg/laplogos_core_bg.wasm");
const destDir = resolve("laplogos_web/public/wasm");
const dest = resolve(destDir, "laplogos_core_bg.wasm");

await mkdir(destDir, { recursive: true });
await copyFile(source, dest);
