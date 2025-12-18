# Project Aix: MVP Development Plan (Week 1)

このドキュメントは、Codex/Copilot への指示書兼進捗チェックリストです。
開発者はこのリストに従って実装を進めます。

## プロジェクトコンテキスト

- **Goal:** Rust 製の UI エンジン(Wasm)をブラウザで動かす。
- **Core:** `aix_core` (Rust / WebAssembly)
- **Web:** `aix_web` (Next.js / TypeScript)
- **Schema:** v1 (ID 必須, 短縮 JSON)

---

## Phase 1: Rust Core の実装 (`aix_core`)

### Task 1.1: 依存関係の定義

- [x] **Target:** `aix_core/Cargo.toml`
- **Spec:**
  - `[lib]` セクション: `crate-type = ["cdylib", "rlib"]`
  - `[dependencies]`:
    - `serde = { version = "1.0", features = ["derive"] }`
    - `serde_json = "1.0"`
    - `wasm-bindgen = "0.2"`
    - `console_error_panic_hook = "0.1"`
  - `[dependencies.web-sys]`:
    - features: `["Document", "Element", "Node", "Window", "HtmlElement", "HtmlButtonElement", "HtmlParagraphElement", "HtmlDivElement", "console"]`

### Task 1.2: スキーマ定義 (Schema v1)

- [x] **Target:** `aix_core/src/lib.rs`
- **Spec:**
  - `serde`, `serde_json::Value`, `std::collections::HashMap` を use する。
  - **Struct `AixPage`**:
    - `v`: u8 (Version)
    - `root`: `AixNode`
  - **Struct `AixNode`** (derive Serialize, Deserialize):
    - `i`: String (rename="i", ID, 必須)
    - `t`: String (rename="t", Type, 必須)
    - `p`: `HashMap<String, Value>` (rename="p", default)
    - `c`: `Option<Vec<AixNode>>` (rename="c", default)
    - `on`: `Option<HashMap<String, Vec<String>>>` (rename="on", default)

### Task 1.3: レンダラーの実装

- [x] **Target:** `aix_core/src/renderer.rs` (新規作成)
- **Spec:**
  - `web_sys` を使用して DOM を生成する。
  - Func `render_node(document: &Document, node: &AixNode) -> Result<Element, JsValue>`
  - Logic:
    - `node.t` が "Box" なら `div` 作成
    - `node.t` が "Txt" なら `p` 作成 (text content = `node.p["val"]`)
    - `node.t` が "Btn" なら `button` 作成 (text content = `node.p["label"]`)
    - `node.i` を `id` 属性にセット
    - `node.c` (children) を再帰的に `append_child`

### Task 1.4: エントリーポイント作成

- [x] **Target:** `aix_core/src/lib.rs`
- **Spec:**
  - `mod renderer;` を追加。
  - Func `init_panic_hook()`: `console_error_panic_hook::set_once()` を呼ぶ。
  - Func `render_aix(json_str: &str, container_id: &str) -> Result<(), JsValue>`:
    - JSON を `AixPage` にパース。
    - `document.get_element_by_id(container_id)` でコンテナ取得。
    - コンテナを空にする (`set_inner_html("")`)。
    - `renderer::render_node` で描画し、コンテナに追加。

---

## Phase 2: Next.js 統合 (`aix_web`)

### Task 2.1: Wasm ビルド (Manual)

- [x] **Action:** Terminal で `cd aix_core && wasm-pack build --target web` を実行。

### Task 2.2: Next.js Config

- [ ] **Target:** `aix_web/next.config.mjs`
- **Spec:**
  - Webpack 設定で `experiments: { asyncWebAssembly: true }` を有効化。

### Task 2.3: Custom Hook

- [ ] **Target:** `aix_web/hooks/useAixEngine.ts`
- **Spec:**
  - `useEffect` で `import("../../aix_core/pkg")` をダイナミックインポート。
  - ロード完了後 `init_panic_hook()` を実行。
  - エンジンオブジェクトを返す。

### Task 2.4: 画面表示

- [ ] **Target:** `aix_web/app/page.tsx`
- **Spec:**
  - `useAixEngine` を使用。
  - `useEffect` でエンジンロード検知後に `render_aix(test_json, "aix-root")` を実行。
  - `test_json`: `{"v":1, "root": {"i":"root", "t":"Box", "p":{"pad":10}, "c":[{"i":"t1","t":"Txt","p":{"val":"Hello AIX"}}]}}`
  - JSX: `<div id="aix-root"></div>`

### Task 2.5: Phase 2 完了確認 (ブラウザ動作検証)

- [ ] **Action:** Terminal (aix_web) で `npm run dev` を実行し、`http://localhost:3000` にアクセス。
- **Check 1 (Console):** ブラウザの開発者ツール(F12)のコンソールに `Aix Engine Loaded!` と表示されていること。
- **Check 2 (Display):** 画面上に `Hello AIX` (JSON で指定した文字) とボタンが表示されていること。
