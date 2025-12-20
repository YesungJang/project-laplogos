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

- [x] **Target:** `aix_web/next.config.mjs`
- **Spec:**
  - Webpack 設定で `experiments: { asyncWebAssembly: true }` を有効化。

### Task 2.3: Custom Hook

- [x] **Target:** `aix_web/hooks/useAixEngine.ts`
- **Spec:**
  - `useEffect` で `import("../../aix_core/pkg")` をダイナミックインポート。
  - ロード完了後 `init_panic_hook()` を実行。
  - エンジンオブジェクトを返す。

### Task 2.4: 画面表示

- [x] **Target:** `aix_web/app/page.tsx`
- **Spec:**
  - `useAixEngine` を使用。
  - `useEffect` でエンジンロード検知後に `render_aix(test_json, "aix-root")` を実行。
  - `test_json`: `{"v":1, "root": {"i":"root", "t":"Box", "p":{"pad":10}, "c":[{"i":"t1","t":"Txt","p":{"val":"Hello AIX"}}]}}`
  - JSX: `<div id="aix-root"></div>`

### Task 2.5: Turbopack/webpack 設定調整

- [x] **Action:** Turbopack デフォルト環境で webpack 設定を併用するため `turbopack: {}` を設定。

### Task 2.5: Phase 2 完了確認 (ブラウザ動作検証)

- [ ] **Action:** Terminal (aix_web) で `npm run dev` を実行し、`http://localhost:3000` にアクセス。
- **Check 1 (Console):** ブラウザの開発者ツール(F12)のコンソールに `Aix Engine Loaded!` と表示されていること。
- **Check 2 (Display):** 画面上に `Hello AIX` (JSON で指定した文字) とボタンが表示されていること。

## Phase 3: Core Logic & State Management (`aix_core`)

### Task 3.1: Action と State の実装

- [x] **Target:** `aix_core/src/store.rs` (新規作成)
- **Spec:**
- `serde`, `serde_json` を使用。
- **Struct `AixState**`:
- `data`: `HashMap<String, String>` (キーと値を保持)。

- **Enum `Action**` (Deserialize):
- `Set { key: String, val: String }` (MVP はこれのみ)。

- **Func `reduce(state: &mut AixState, action: Action)**`:
- Action に基づき State を更新する。

### Task 3.2: Engine Struct 化 (State 保持)

- [x] **Target:** `aix_core/src/lib.rs`
- **Spec:**
- `static` 変数ではなく、`#[wasm_bindgen]` 構造体で状態を持つ形に変更。
- **Struct `AixEngine**`:
- `state`: `store::AixState`。
- `page`: `AixPage` (現在の UI ツリー定義)。

- **Func `new(initial_json: &str) -> AixEngine**`:
- 初期化時に JSON をパースして保持。

- **Func `get_state_json(&self) -> String**`:
- 現在の State を JSON 文字列で返す (デバッグ用)。

### Task 3.3: Dispatcher と Event Handling

- [ ] **Target:** `aix_core/src/lib.rs`
- **Spec:**
- `AixEngine` にメソッド追加。
- **Func `dispatch(&mut self, action_json: &str) -> Result<(), JsValue>**`:
- Action JSON をパースして `reduce` を呼ぶ。
- **重要:** State 更新後、UI ツリー内の変数（例: `$count`）を置換して再レンダリング処理を呼ぶ（Task 3.4 へ続く）。
- ※MVP では変数がなければ「再描画」だけトリガーする。

### Task 3.4: Re-rendering ロジック

- [ ] **Target:** `aix_core/src/renderer.rs`
- **Spec:**
- `render_node` を修正し、イベントリスナーを付与できるようにする。
- **Logic:**
- `node.on` ("click" 等) があれば、Rust 側の `dispatch` を呼ぶクロージャを `addEventListener` で設定。
- ※Wasm から Rust のメソッドを呼ぶため、`Closure` を適切に扱う必要がある。

- **Func `update_dom(&self, container_id: &str)**`:
- 既存の要素をクリアして再描画する（Virtual DOM なしの単純実装）。

---

## Phase 4: Interaction & Visualizer (`aix_web`)

### Task 4.1: Debug View コンポーネント

- [ ] **Target:** `aix_web/components/DebugView.tsx` (新規作成)
- **Spec:**
- Props: `engine: AixEngine | null`, `refreshTrigger: number`
- **Layout:**
- 画面右側（または下部）に固定表示。
- "State Inspector" と "Action Log" を表示。

- **Logic:**
- `engine.get_state_json()` を定期的に（またはトリガーで）呼び出して表示。

### Task 4.2: Engine Hook のアップグレード

- [ ] **Target:** `aix_web/hooks/useAixEngine.ts`
- **Spec:**
- `AixEngine` インスタンスを React State (`useRef` または `useState`) で保持するように変更。
- `dispatch` ラッパー関数を作成し、Action 発行後に React 側の再レンダリング（`refreshTrigger` 更新など）をトリガーして Debug View を更新させる。

### Task 4.3: レイアウト統合と動作確認

- [ ] **Target:** `aix_web/app/page.tsx`
- **Spec:**
- 画面を 2 カラムに分割 (Grid or Flex)。
- **Left:** `<div id="aix-root">` (アプリ本体)
- **Right:** `<DebugView />` (内部状態可視化)
- **Test Data:**
- 初期 JSON に Button を配置。
- `on: { "click": ["Set", "count", "random_val"] }` のような定義を含める。

### Task 4.4: Phase 4 完了確認 (MVP サイクル検証)

- [ ] **Action:** ブラウザでボタンをクリック。
- **Check 1 (Interaction):** ボタンを押してもエラーが出ないこと。
- **Check 2 (State Update):** ボタン押下後、右側の Debug View の State 表示が更新されること。
- **Check 3 (Re-render):** (もし変数置換を実装していれば) 画面上の文字が変わること。実装していなければ、コンソールログで State 変化を確認。
