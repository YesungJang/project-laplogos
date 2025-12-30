# Preview Release & Deployment

## ビルド成果物と手順

- wasm 生成: `wasm-pack build --target web`（`laplogos_core`）
- wasm 配置: `scripts/prepare-wasm.mjs` で `laplogos_web/public/wasm/` へコピー
- web build: `npm run build`（`laplogos_web`）

## 環境差分（local/CI/prod）

- Node.js 20.9+ で統一
- SSR は Node.js のみ（Edge は対象外）
- wasm パスは `LAPLOGOS_WASM_PATH` で上書き可能

## CI 再現性

- core: `cargo test`
- web: `npm run lint`
- version check: Git tag と `Cargo.toml` / `package.json` の整合性

## Preview デプロイ（1 つの正解）

- Node SSR で `npm run build` → `npm run start`
- `public/wasm/laplogos_core_bg.wasm` を必ず配置
