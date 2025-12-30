# Project laplogos (Preview)

**"Human-Unreadable, Machine-Optimal"**
AI ネイティブ時代の次世代 UI ランタイム＆開発言語プロジェクト。

## コンセプト

- **AI First:** 人間の可読性より、AI の生成効率（トークン削減）を最優先。
- **No Build Time:** AI が生成したデータを Rust エンジンが即座に実行。
- **Safety:** 任意のコード実行を禁止し、定義された Action のみを許可。

## 技術スタック (Rust-Centric)

- **Core Engine:** Rust (WebAssembly)
- **Host App:** Next.js (React)
- **Schema:** laplogos Schema v1 (JSON based)

## ディレクトリ構成

- `laplogos_core/`: Rust 製ランタイムエンジン (Wasm)
- `laplogos_web/`: フロントエンド (Next.js) + デバッグ可視化ツール

## 対象環境

- Node.js 20.9+
- Browser: Chrome/Edge/Firefox 111+、Safari 16.4+
- Rust（`cargo` / `wasm-pack`）

## 開発環境セットアップ

### 起動手順（CSR）

1. **Rust エンジンのビルド**
   ```bash
   cd laplogos_core
   wasm-pack build --target web
   ```
2. **wasm の配置**
   ```bash
   node scripts/prepare-wasm.mjs
   ```
3. **Web 起動**
   ```bash
   cd laplogos_web
   npm install
   npm run dev
   ```

### SSR での起動

```bash
cd laplogos_web
npm run build
npm run start
```

`LAPLOGOS_WASM_PATH` で SSR の wasm パスを上書きできます。

## 設定（Client）

- `laplogos_web/public/laplogos-config.json` で `allowed_actions` / `allowed_hosts` を設定
- 環境変数で上書き可能
  - `NEXT_PUBLIC_LAPLOGOS_ALLOWED_ACTIONS`
  - `NEXT_PUBLIC_LAPLOGOS_ALLOWED_HOSTS`

## ドキュメント

- 互換性/Versioning: `docs/preview/compatibility.md`
- セキュリティ: `docs/preview/security.md`
- 観測性/デバッグ: `docs/preview/observability.md`
- パフォーマンス: `docs/preview/performance.md`
- 配布/デプロイ: `docs/preview/release.md`
- Schema v1: `docs/schema-v1.md`
- SSR/CSR/Hydration: `docs/ssr-csr-hydration.md`
- サンプル: `docs/samples/`
