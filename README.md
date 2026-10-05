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
- SSR 参照の設定パスは `LAPLOGOS_CONFIG_PATH` で上書き可能

## テスト

- Web:
  - `npm run lint`
  - `npm run test:node`
  - `npm run test:e2e`（Playwright。必要なら `npx playwright install chromium`）
- Core:
  - `cargo test -p laplogos_core`

## ドキュメント

- 互換性/Versioning: `docs/preview/compatibility.md`
- セキュリティ: `docs/preview/security.md`
- 観測性/デバッグ: `docs/preview/observability.md`
- パフォーマンス: `docs/preview/performance.md`
- 配布/デプロイ: `docs/preview/release.md`
- Schema v1: `docs/schema-v1.md`
- SSR/CSR/Hydration: `docs/ssr-csr-hydration.md`
- サンプル: `docs/samples/`

## Development

ローカルで実行できる検証コマンドをまとめます。
とくに断りがない限り、コマンドはすべて**リポジトリのルート**から実行します。
プルリクエストを作成する前に、ここに記載したローカル検証コマンドをすべて実行してください。

### 事前準備

Web 側の検証（`test:node` / `lint` / `build` / `test:e2e`）は、
ビルド済みの wasm パッケージと `laplogos_web` の依存関係に依存します。
初回、または `laplogos_core/` を変更した後は次を実行してください。

```bash
cd laplogos_core
wasm-pack build --target web
cd ..
node scripts/prepare-wasm.mjs
npm --prefix laplogos_web install
```

- `laplogos_web/package.json` は `laplogos_core` を `file:../laplogos_core/pkg` として参照するため、`wasm-pack build` は `npm install` より先に必要です。
- `scripts/prepare-wasm.mjs` はルートからの相対パスを解決するので、ルートで実行してください。

`npm run test:e2e` を実行する場合は、Playwright のブラウザも取得します。

```bash
cd laplogos_web
npx playwright install chromium
```

### Core（Rust）

ルートに Cargo ワークスペースはないため、`--manifest-path` でクレートを指定します。

```bash
cargo test --manifest-path laplogos_core/Cargo.toml
```

ディレクトリを移動して実行する場合:

```bash
cd laplogos_core
cargo test
```

### Web（Next.js）

`laplogos_web/package.json` に定義されたスクリプトを実行します。

```bash
npm --prefix laplogos_web run test:node
npm --prefix laplogos_web run lint
npm --prefix laplogos_web run build
npm --prefix laplogos_web run test:e2e
```

ディレクトリを移動して実行する場合:

```bash
cd laplogos_web
npm run test:node
npm run lint
npm run build
npm run test:e2e
```

### 各コマンドの内容

| コマンド | 実行内容 |
| --- | --- |
| `cargo test` | `laplogos_core` のユニット/統合テスト（`laplogos_core/tests/`） |
| `npm run test:node` | `node --test --import tsx tests/*.test.ts`（SSR/CSR の Node テスト） |
| `npm run lint` | ESLint（`laplogos_web/eslint.config.mjs`） |
| `npm run build` | `next build --webpack`（本番ビルド） |
| `npm run test:e2e` | Playwright（`laplogos_web/e2e/`） |

- `npm run test:e2e` は `playwright.config.ts` の `webServer` 設定により `npm run dev` を自動起動し、`http://localhost:3000` に対してテストします。すでに開発サーバーを起動している場合はそれを再利用します。
