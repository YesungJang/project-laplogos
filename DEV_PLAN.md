# Project laplogos: Master Development Plan (dev_plan.md)

**Project Goal:** "Human-Unreadable, Machine-Optimal"  
AI ネイティブ時代の次世代 UI ランタイム。Web での完成度（Stage 2）を高めてから Native 展開（Stage 3）へ進む。

- **Core:** `laplogos_core` (Rust / WebAssembly)
- **Web:** `laplogos_web` (Next.js / TypeScript)
- **Schema:** v1 (Strict ID, Short keys)

---

## Global Policies (MVP Non-Negotiables)

### 1) Error & Validation Policy

- **Schema validation（構造/ID/loop 制約違反）**は **Fatal** とする  
  → 画面は更新せず、**Error View** を表示し、Debug/Error Log に記録する。
- **Action validation（不正/未知 Action）**は **Non-fatal** とする  
  → `console.warn` + Debug/Error Log へ記録し、その Action だけ **Drop**。
- **Patch apply 失敗（DOM が見つからない等）**は **Non-fatal → Fallback** とする  
  → Debug/Error Log に記録し、**フル再描画（root 置換）**にフォールバックする。

### 2) Determinism Policy

- 同じ `(Schema, State)` から生成される **Materialized Tree** と **HTML** は常に同一であること。

### 3) Root Policy

- MVP では `LaplogosPage.root` は **常に 1 ノード**（Materialize 後も 1 ノード）
  - root 直下で `for` を許可しない（必要になったら v2 で検討）
  - root に `if` を付けるのは可。ただし `if=false` の場合は **Fatal**（空画面ではなく Error View）

---

## ✅ Stage 1: Foundation (Web MVP) [COMPLETED]

### Phase 1: Rust Core Foundation

- [x] Task 1.1: Dependencies (`serde`, `wasm-bindgen`, `web-sys`) setup.
- [x] Task 1.2: Schema v1 Definition (`LaplogosPage`, `LaplogosNode`).
- [x] Task 1.3: Basic Renderer (`Box`, `Txt`, `Btn`).
- [x] Task 1.4: Entry Point (`render_laplogos`).

### Phase 2: Next.js Integration

- [x] Task 2.1: Wasm Build setup (`wasm-pack`).
- [x] Task 2.2: Next.js Config (`asyncWebAssembly`).
- [x] Task 2.3: Custom Hook (`useLaplogosEngine`).
- [x] Task 2.4: Screen Display (Hello laplogos).

### Phase 3: State & Action Logic

- [x] Task 3.1: Store & Action Struct (`LaplogosState`).
- [x] Task 3.2: Engine Struct (`LaplogosEngine`).
- [x] Task 3.3: Dispatcher Implementation.
- [x] Task 3.4: Re-rendering Logic.

### Phase 4: Debug Visualization

- [x] Task 4.1: Debug View Component.
- [x] Task 4.2: Engine Hook Upgrade (Ref-based).
- [x] Task 4.3: Layout Integration.
- [x] Task 4.4: MVP Cycle Verification.

---

## 🛠 Stage 2: Application Core (Web App)

**Goal:** 実用的なアプリケーションロジックの構築。仕様の曖昧さを排除し、堅牢な基盤を作る。

---

## Phase 5: Restricted Dynamic Binding (`laplogos_core`)

**方針:**  
式評価を徹底排除し、構造化された Action 定義と厳格なループ制約により、ランタイムの肥大化とバグを防ぐ。

---

### Task 5.0: Action Normalization (ActionSpec)

- [x] **Target:** `laplogos_core/src/store.rs` & `laplogos_core/src/lib.rs`
- [x] **Spec:**

**Struct `ActionSpec`:**

- `t: String` (type) - 小文字統一（`set`, `inc`, `fetch` など）
- `d: HashMap<String, serde_json::Value>` (data)

**Normalization:**

- JSON が文字列 `"inc"` の場合 → `ActionSpec { t:"inc", d:{} }` として扱う
- JSON がオブジェクトの場合 → そのまま `ActionSpec` にマッピング
- それ以外（配列/数値/null）→ Invalid → Drop（warn + error log）

**Validation Logic（dispatch 前）:**

- `set`
  - `d["key"]` (String) 必須
  - `d["val"]` **OR** `d["val_from"]` (String) の **片方のみ必須（XOR）**
  - **Constraint:** `d["val_from"]` は `"event.value"` **のみ許可**。それ以外は Invalid → Drop
- `inc`
  - `d["key"]` (String) 必須
  - `d["by"]` (Number) 任意（default: 1）
- `fetch` は Phase 7 で validation を追加（Phase 5 では未知扱いでも可）
- Unknown/Invalid: `console.warn` + Debug/Error Log に記録して Drop

**No-op Optimization:**

- `set` 実行時、State の既存値と新値が同一なら `rerender` をスキップ

---

### Task 5.1: Strict Variable Substitution (No Expr)

- [x] **Target:** `laplogos_core/src/materialize.rs`（Task 5.5 で新設）または `renderer.rs`
- [x] **Spec:**

**Helper:**

- `fn resolve_text(val: &Value, state: &LaplogosState, scope: &MaterializeScope) -> String`
- 文字列かつ `"$key"` (完全一致) の場合のみ State から取得
- 部分一致 (`"Hi $name"`) / `${}` / 演算は **非対応**（そのまま表示）

**Scope（置換対象）:**

- `Txt.p["val"]`
- `Btn.p["label"]`
- `Inp.p["val"]`

---

### Task 5.2: Strict Boolean If

- [x] **Target:** `laplogos_core/src/materialize.rs`（推奨）または `renderer.rs`
- [x] **Spec:**
- **Logic:** State 値が **Boolean(true)** の場合のみ通す
- `"true"`(文字列), `1`, `null`, 未定義キーは全て false 扱い
- `if` 判定が非 bool だった場合は warn を残す（ただし挙動は false）

---

### Task 5.3: Global ID Constraint

- [x] **Target:** `laplogos_core/src/materialize.rs`（推奨）+ `tests/conformance.rs`
- [x] **Spec:**

**Global Schema Constraint（Fatal）:**

- `LaplogosNode.i` は常に **`[A-Za-z0-9_:-]+`** を満たすこと
- 違反が 1 つでもあれば **Fatal**（Error View + log、DOM 更新停止）

> NOTE: `:` は `{parent}:{item}` のために許可する。

---

### Task 5.4: Action-based Input (No Magic Binding)

- [x] **Target:** `laplogos_core/src/renderer.rs`
- [x] **Spec:**
- `Inp` ノード（`<input>`）実装
- **Validation:** `Inp.p["val"]` は `"$key"` 形式必須（完全一致）。違反は warn + no-op
- **Action Generation:**
  - `set` 先キーは `Inp.p["val"]` から先頭 `$` を除去して決定
  - 発行 Action: `{ "t": "set", "d": { "key": "<stripped_key>", "val_from": "event.value" } }`
- `$event.value` 等の式評価はしない。`val_from=="event.value"` を見て Rust 側で `event.target.value` を抽出して `set` を実行する。

---

### Task 5.5: Materialize View Tree (Flatten 対応 / Diff 対象の解決済みツリー) **[必須]**

> Diff が「$count / if / for を含む Schema ツリー」を比較すると更新が反映されないため、  
> **Materialize（解決済みツリー）を必ず生成し、それを Diff/Render の基準にする。**

- [x] **Target:** `laplogos_core/src/materialize.rs`（新規）
- [x] **Spec:**

#### Data Structures

- `struct MaterializeScope {`
  - `item: Option<String>` // "$item" 用（ループ内のみ）
  - `key: Option<String>` // "$key" 用（ループ内のみ）
  - `}`

#### Functions (重要: “複数ノードを返せる” 形にする)

- `fn materialize_many(node: &LaplogosNode, state: &LaplogosState, scope: &MaterializeScope) -> Result<Vec<LaplogosNode>, LaplogosError>`
  - 通常ノード: `Ok(vec![one])`
  - `if=false`: `Ok(vec![])`（ノードを消す）
  - `for`: `Ok(vec![expanded...])`（複数ノード展開）
- `fn materialize_one_root(root: &LaplogosNode, state: &LaplogosState) -> Result<LaplogosNode, LaplogosError>`
  - Root Policy により **必ず 1 ノード** を返す
  - root の `if=false` は **Fatal**（空ツリーにしない）

#### Loop Constraints (MVP / Fatal)

- 対象配列は **String 配列** または **Integer 配列** のどちらか（混在禁止）
- 値の文字列化が `[A-Za-z0-9_:-]+` を満たすこと（満たさない場合 Fatal）
- 値の重複禁止（重複があれば Fatal）
- ループ内参照は `"$item"` と `"$key"` のみ（それ以外は通常の `"$key"` 置換として扱わない）

#### Materialize Rules

- `if_` 判定を適用して枝刈り
- `for_` を展開し、展開後ノードから `for_` は消す（None）
- `Txt.val / Btn.label / Inp.val` のみ `resolve_text` を適用し、**解決済み文字列を materialized node に書き戻す**
- 生成 ID: `format!("{}:{}", parent_i, item_value_str)`（`:`は許可済み）

---

## Phase 6: Client-Side Smart Diffing (`laplogos_core`)

**方針:**  
DOM の破壊的更新を最小限にし、フォーカス維持とパフォーマンス確保を行う。  
Diff 対象は必ず **Materialized Tree**。

---

### Task 6.0: Engine Loop Update (Materialize → Diff → Patch)

- [x] **Target:** `laplogos_core/src/lib.rs`
- [x] **Spec:**
- `LaplogosEngine` は `last_materialized: Option<LaplogosNode>` を保持する
- `dispatch` 後のレンダリングフロー:
  1. `new_materialized = materialize_one_root(page.root, state)`
  2. `patches = diff(last_materialized, new_materialized)`
  3. `apply_patches(patches)`（失敗時は fallback）
  4. `last_materialized = new_materialized`

---

### Task 6.1: Diff Engine & Patch Definition

- [x] **Target:** `laplogos_core/src/diff.rs`（新規）
- [x] **Spec:**

**Enum `Patch`:**

- `Append { parent_id: String, node: LaplogosNode }`
- `Remove { id: String }`
- `Replace { id: String, node: LaplogosNode }`
- `UpdateText { id: String, val: String }`
- `UpdateAttr { id: String, key: String, val: String }`
- `ReorderChildren { parent_id: String, ordered_ids: Vec<String> }`

**Diff 対象:**

- `diff(old: &LaplogosNode, new: &LaplogosNode) -> Vec<Patch>` は **materialized tree 同士**で比較する

**Rules (MVP):**

- `id` 同一で `t` が違う → `Replace`
- `on`（イベント定義）が変わった → **安全のため `Replace`**
- props 差分は `UpdateText/UpdateAttr` を優先（ただし安全にできない場合は `Replace`）
- children は ID で同定
  - 集合同一・順序のみ違う → `ReorderChildren`
  - 追加/削除 → `Append` / `Remove`
  - 複雑差分は保守的に `Replace` でも可

---

### Task 6.2: Patch Applier & DOM Identification

- [x] **Target:** `laplogos_core/src/renderer.rs`
- [x] **Spec:**

**Preparation:**

- `render_node` は全要素に `data-laplogos-id="{node.i}"` を付与する
- 可能ならルート要素にも `data-laplogos-root="1"` を付与（fallback 置換を容易にする）

**Lookup:**

- `document.querySelector(&format!("[data-laplogos-id=\"{}\"]", id))` を使う
- `id` は Global ID Constraint により selector-safe

**Fallback Policy:**

- 対象 DOM が見つからない / 操作失敗 → Error Log 記録 → **フル再描画で root を置換**（破綻状態を回避）

---

### Task 6.3: Focus Guard

- [x] **Target:** `laplogos_core/src/renderer.rs`
- [x] **Spec:**
- `UpdateAttr` で `value` 更新する際、
  - 対象要素が `document.activeElement` と一致する場合は **スキップ**
- 目的: 入力中の value 上書き/フォーカス喪失を防ぐ

---

## Phase 7: Capability & Security (`laplogos_core`)

**方針:**  
セキュリティ基盤を先に整え、その上で通信機能を実装する。

---

### Task 7.1: Capability System

- [x] **Target:** `laplogos_core/src/store.rs` & `laplogos_core/src/lib.rs`
- [x] **Spec:**
- `struct LaplogosConfig { allowed_actions: Vec<String>, allowed_hosts: Vec<String> }`
- `LaplogosEngine::init(config)` で注入可能
- `dispatch` 直前で `allowed_actions` をチェック（未許可は Drop + log）
- URL チェックヘルパー（`allowed_hosts` と照合）を実装

---

### Task 7.2: Async Fetch Action

- [x] **Target:** `laplogos_core/src/lib.rs`
- [x] **Spec:**

**Schema (`fetch` ActionSpec.d):**

- `url: String` 必須
- `on_success: JSON Array`（Runtime parse to `Vec<ActionSpec>`）
- `on_fail: JSON Array`（Runtime parse to `Vec<ActionSpec>`）

**Runtime:**

- `allowed_hosts` に合致しない host は Drop + log
- `web_sys::fetch` を使用
- 完了後、パース済みの `Vec<ActionSpec>` を **順に dispatch**（順序保証）
- `dispatch` は capability を必ず通る（内部発行でも例外なし）

---

## Phase 8: Reliability & Testing (`laplogos_core`)

**方針:**  
仕様準拠を保証するテスト基盤を構築（クラッシュしない / 仕様がブレない）。

---

### Task 8.1: Conformance Suite

- [x] **Target:** `tests/conformance.rs`
- [x] **Spec:**
- `tests/data/` に `valid_*.json` / `invalid_*.json` を配置
- 一括テストランナーを作成
  - Valid は成功
  - Invalid（ID 文字種違反・loop 重複・loop 型混在・if 非 bool など）は **Fatal として Error になる**こと
  - panic しないこと

---

### Task 8.2: Deterministic Render Test (Snapshot)

- [x] **Target:** `tests/snapshot.rs`
- [x] **Spec:**
- DOM を触らずに比較できる形にする
  - `materialize_one_root` → `render_to_string(materialized_root)` の純関数パス
- 「特定の State + JSON」で常に同じ HTML が生成されることを検証

---

## 🧱 Stage 2.5: Web Production Readiness

**Goal:** Web 版として「完成」させる（トークン効率化 / SPA 対応 / SSR+Hydration による SEO）。  
**Principle:** Core（Materialize / Diff / Patch）は「正規化済みのきれいなツリー」だけを扱い、複雑さを前処理（Normalize）に隔離する。

---

## Phase 9: Token Optimization（Normalization Layer）

**方針:** 圧縮表現（Def/Ref, Style Dict）を **Normalize（正規化）で展開**し、以降のコアは常に **Schema v1（正規）**だけを処理する。

### Task 9.1: Packed Schema（Def/Ref + Style Dict）定義

- [x] **Target:** `laplagos_core/src/lib.rs`（Schema 拡張） or `laplagos_core/src/schema_packed.rs`（新規）
- [x] **Spec:**
- `LaplagosPage`（または PackedPage）に以下を追加（存在すれば packed とみなす）
  - `defs: HashMap<String, LaplagosNode>`（定義ノード。`i` は持たない前提）
  - `styles: HashMap<String, HashMap<String, String>>`（共通スタイル辞書。値は必ず String）
- `LaplagosNode` に以下を optional で追加（packed 表現用）
  - `r: Option<String>`（Ref: defs 参照 ID）
  - `st: Option<String>`（Style 参照 ID）
  - `o: Option<HashMap<String, serde_json::Value>>`（override。`p` の上書き専用）
- **Validation（Fatal）**
  - `r` が defs に無い / `st` が styles に無い → Fatal
  - `o` が `p` 以外（children 等）を上書きしようとする → Fatal
  - `styles[*]` の value が String 以外 → Fatal

### Task 9.2: Normalize 実装（Packed → 正規 Schema v1）

- [x] **Target:** `laplagos_core/src/normalize.rs`（新規）
- [x] **Spec:**
- `fn normalize_page(input: serde_json::Value) -> Result<LaplagosPage, LaplagosError>`
- 入力が通常 v1 ならそのままパース。packed なら以下を実施:
  1. `r` を deep clone で展開（参照側ノードの `i` を必須扱い）
  2. `o` があれば `p` に **上書きマージ**（`p` 以外は上書き禁止）
  3. `st` があれば `styles[st]` を `p` に **不足分のみマージ**（既存 `p` を優先）
  4. 生成結果から `defs/styles/r/st/o` を全て除去し、**正規 v1 のみ**にする
- **Note:** Global ID Constraint や Loop 制約など “既存の Schema ルール” は緩和しない（Normalize は展開だけ）。

### Task 9.3: Normalize Conformance テスト追加

- [x] **Target:** `tests/conformance.rs` + `tests/data/`
- [x] **Spec:**
- `valid_packed_*.json` / `invalid_packed_*.json` を追加
- invalid 例:
  - 未定義 `r` / 未定義 `st`
  - `o` が `p` 以外を書き換え
  - `styles` の値が String 以外
- 期待挙動: Fatal で落ちる（ただし panic しない / ログが残る）

### Task 9.4: Engine Loop 先頭に Normalize を挿入（重要）

- [x] **Target:** `laplagos_core/src/lib.rs`
- [x] **Spec:**
- **Task 6.x（Materialize→Diff→Patch→DOM）のフローの先頭**に `normalize_page` を追加する
- Engine が保持するページは `normalized_page: LaplagosPage` に統一
- `set_page(...)` / 初期化入口で Normalize → 以降は normalized_page のみを参照

---

## Phase 10: Router / SPA（Core minimal / Web 主導）

**方針:** Core は「遷移要求（nav）」と「通知（callback）」だけを持つ。  
実際のルーティング（Next.js router、履歴管理、ページ JSON 取得）は **laplagos_web** が担当。

### Task 10.1: `nav` ActionSpec 追加（Core 側は通知のみ）

- [x] **Target:** `laplagos_core/src/store.rs` & `laplagos_core/src/lib.rs`
- [x] **Spec:**
  - `t: "nav"`
  - `d["to"]: String` 必須
  - Capability:
    - `allowed_actions` に `nav` が無ければ Drop
    - `to` が外部 URL の場合、host を `allowed_hosts` と照合（不一致は Drop）
  - 実行:
    - Core は遷移しない。`on_nav(to)` コールバックを呼ぶ（未設定なら warn + Drop）

### Task 10.2: `Lnk` ノード（リンク UI）

- [x] **Target:** `laplagos_core/src/lib.rs`（Node type 追加） & `laplagos_core/src/renderer.rs`
- [x] **Spec:**
  - `t: "Lnk"`
  - `p["href"]: String` 必須
  - `p["label"]: String` 必須
  - Render: `<a href="...">label</a>`
  - click で `preventDefault` → `dispatch(nav{to:href})`
  - **推奨:** `Task 5.1 resolve_text` の対象に `Lnk.p["label"]` を追加（Phase 10 実装時で OK）

### Task 10.3: Web 側 Router 統合（Next.js）

- [x] **Target:** `laplagos_web`（`useLaplagosEngine` など）
- [x] **Spec:**
  - Engine init 時に `on_nav` を注入
  - `on_nav(to)`:
    - app 内 path → `router.push(to)`
    - 外部 URL → `window.location.assign(to)`（Core 側で allowed_hosts を通っている前提）
  - route 変更時に schema を取得し `engine.set_page(page_json)` を呼ぶ

### Task 10.4: History / Popstate 最低限同期

- [x] **Target:** `laplagos_web`
- [x] **Spec:**
  - 初期表示: 現在 path から schema 解決して set_page
  - 戻る/進む: path 変化を検知して schema 再ロード → set_page

---

## Phase 11: SSR & Hydration（失敗時フォールバック）

**方針:** SSR で SEO 用 HTML を生成し、クライアントで Hydration（イベント接続）を行う。  
Hydration 失敗時は **フル再描画（root replace）にフォールバック**して「画面真っ白」を回避する。

### Task 11.1: SSR 向け `render_to_string` API（Core）

- [x] **Target:** `laplagos_core/src/lib.rs`
- [x] **Spec:**
- `render_laplagos_to_string(page_json: JsValue, state_json: JsValue) -> Result<String, JsValue>`
- 内部フロー（純関数）:
  1. Normalize（Phase 9）
  2. Materialize（Task 5.5 がある前提）
  3. HTML 文字列化（サーバ用レンダラー）

### Task 11.2: Next.js SSR 組み込み + JSON 埋め込み（XSS 対策を明記）

- [x] **Target:** `laplagos_web`（SSR 実装箇所）
- [x] **Spec:**
- サーバで `render_laplagos_to_string` を呼び HTML 出力
- 初期データ（page/state）を `<script type="application/json">` に埋め込む
- **XSS 対策（必須）**
  - `JSON.stringify(data)` の結果に含まれる `<` を `\\u003c` に置換して埋め込む
  - （例）`safe = json.replace(/</g, "\\u003c")`

### Task 11.3: Hydration（イベントのみ接続）+ 不一致時フォールバック

- [x] **Target:** `laplagos_core/src/renderer.rs` + `laplagos_web`
- [x] **Spec:**
- クライアント起動時:
  1. bootstrap JSON を読む
  2. Normalize → Materialize して `materialized_tree` を作る
  3. **Hydration 専用 `bind_events(materialized_tree)`** を実行（DOM は作らない）
     - `data-laplagos-id` で DOM を特定してイベントを貼る
     - DOM が見つからない / 失敗 → Error Log 記録 → **root replace でフル再描画**
  4. 成功時: `last_materialized = materialized_tree` として通常ループへ

### Task 11.4: Hydration 不安定要因の吸収（最小）

- [x] **Target:** `laplagos_web`
- [x] **Spec:**
  - 拡張機能等による微差分で警告が出る箇所は必要最小限で `suppressHydrationWarning` を使う
  - ただし UI の正当性は Core の Materialize/Diff/Patch を正とする（Hydration は補助）

---

## Phase 12: Testing Strategy & Harness (Web-First)

**Goal:** Web 版（SSR/CSR/Diff/Patch/Router）が今後の変更で壊れないよう、**高速で再現性の高いテスト基盤**を先に確立する。
**Strategy:** テストは 3 レイヤに分離し、**9 割は純 Rust/Node 寄せ**、DOM が絡む領域だけを **ブラウザ（E2E or wasm-bindgen-test）**で薄く担保する。

### Layering Policy (重要)

1. **Layer 1: Core Unit Tests (Pure Rust / Fast)**

   - 対象: `laplogos_core` の **DOM 非依存**ロジック
   - 目的: 回帰の大半をここで潰す（最重要・最頻）

2. **Layer 2: SSR Integration Tests (Node / Medium)**

   - 対象: SSR で使う実経路（`render_laplogos_to_string` など）
   - 目的: “本番呼び出し” の整合性保証（少数のゴールデンケース）

3. **Layer 3: Browser Behavior Tests (E2E / Slow)**

   - 対象: DOM 適用・フォーカス維持・Hydration 失敗フォールバック等、ブラウザ実挙動が必要な領域
   - 目的: 最小ケースで「壊れると致命的」だけを押さえる（増やさない）

---

### Task 12.1: Core Unit Test Harness (Pure Rust)

- [x] **Target:** `laplogos_core/tests/`

- [x] **Spec:**

  - `normalize` / `schema validation` / `materialize` / `diff` のテストを**純 Rust で実行**できる構成に統一する
  - `apply_patch` や `web_sys` 依存は **Layer 1 には入れない**

- [x] **Deliverables:**

  - `tests/materialize.rs`
  - `tests/diff.rs`
  - `tests/normalize.rs`
  - `tests/schema_validation.rs`

---

### Task 12.2: Golden Snapshot Tests (Materialize Output)

- [x] **Target:** `laplogos_core/tests/snapshot_materialize.rs`
- [x] **Spec:**

  - 入力: `schema(JSON pre-normalize)` + `state`
  - 期待: `normalize -> materialize` の結果（**Materialized View Tree**）が常に同一
  - **Case must include:** `if`, `for (primitive)`, `Ref/Def`, `resolve_text("$key")`
  - Snapshot 形式は **JSON** または **安定化した文字列表現**（順序・空白が固定される形）で保存する

---

### Task 12.3: Patch Invariants Tests (Diff Output Sanity)

- [x] **Target:** `laplogos_core/tests/patch_invariants.rs`
- [x] **Spec:** `diff(old_tree, new_tree)` の出力 `Vec<Patch>` に対し、不変条件を検証する

  - `Patch` の `id` / `parent_id` は必ず **Global ID Constraint** を満たす
  - `ReorderChildren.ordered_ids` は **重複なし**かつ **同一親配下の既存子集合**である
  - `Append/Remove/Replace` の適用順序に依存して壊れないよう、出力のルールを固定（最低限の順序規約を明文化）

- [x] **Note:** 完全な DOM 適用はここではしない（Layer 1 に留める）

---

### Task 12.4: SSR Integration Tests (Node)

- [x] **Target:** `laplogos_web/tests/ssr_integration.test.ts`（または同等の Node 実行テスト）

- [x] **Spec:**

  - SSR 本番と同じ経路で `render_laplogos_to_string` を呼び出し、HTML を取得
  - **Inputs:** `page`（URL パス）・初期 schema・初期 state
  - **Assertions:**

    - SSR HTML に `data-laplogos-id` が付与されている
    - SSR が埋め込む JSON は **`<` が `\u003c` にエスケープ**されている（XSS 対策）
    - `?page=` の page 文字列は **URL パスそのまま**が入力になっている（SSR/CSR 一致）

- [x] **Case Count:** まずは **1〜2 ケース**で十分（増やし過ぎない）

---

### Task 12.5: CSR Bootstrap Consistency Tests (Node)

- [x] **Target:** `laplogos_web/tests/csr_bootstrap_consistency.test.ts`
- [x] **Spec:**

  - SSR が埋め込む `bootstrap(schema/state)` を取り出し
  - CSR 側と同じ入力で `normalize -> materialize` した結果が、期待するツリーに一致する
  - 目的は **SSR/CSR の入力一致** と **正規化前埋め込み統一**を固定すること

---

### Task 12.6: Browser E2E Tests (Minimum Set)

**方針:** ここは増やさない。致命箇所だけをゴールデンケースで押さえる。
（推奨: Playwright。wasm-bindgen-test でも可だが、SSR/Router 含めた一気通貫は E2E が楽。）

- [x] **Target:** `laplogos_web/e2e/`

- [x] **Cases (must-have 3):**

  1. **Focus Guard:** `Inp` 入力中に state 更新が走っても、フォーカスと入力値が壊れない
  2. **ReorderChildren:** ループ表示の並び替えで DOM node instance が維持され、入力が飛ばない
  3. **Hydration Failure Fallback:** Hydration 失敗時に root 置換へフォールバックし、白画面にならない

- [x] **Assertions:**

  - Input の `value` が意図せず巻き戻らない
  - `document.activeElement` が維持される
  - Hydration 失敗時でも画面が表示され、エラー表示 or フル再描画が動く

---

### Task 12.7: CI Test Split (Fast/Medium/Slow)

- [x] **Target:** CI 設定（GitHub Actions 等）
- [x] **Spec:**

  - **Fast:** Layer 1（純 Rust テスト）を PR ごとに必ず実行
  - **Medium:** Layer 2（SSR/CSR Node テスト）を PR ごとに実行
  - **Slow:** Layer 3（E2E）は PR では任意（または main マージ時/夜間）にして、開発速度を落とさない

---

## Notes (Implementation Constraints)

- Layer 1 のテスト対象は **必ず DOM 非依存**に保つ（`web_sys` を混ぜない）
- `materialize` の結果は Diff/Render の基礎なので、Snapshot は **最重要資産**として扱う
- E2E は “増やさない” をルール化し、壊れると致命的な箇所だけを維持する

---

## Design Decisions（Preview Defaults）

> Preview 版は「最短でリリース可能状態にする」ことを優先し、仕様の曖昧さと手戻りを避けるために、以下を **デフォルト方針として固定**する。
> 例外が必要になった場合のみ、このセクションを更新する。

### 1) Compatibility / Versioning

- **Schema 破壊的変更の定義（= schema `v` を上げる）**

  - 必須フィールド追加、型変更、意味変更（解釈変更）
  - ID 制約（許可文字種）の変更
  - Diff/Patch の意味が変わり、同一入力で DOM 結果が変わる変更

- **破壊的でない変更（= schema `v` 据え置き）**

  - 新しい Node type (`t`) の追加、任意 props の追加、Debug 情報の追加（既存解釈を変えない範囲）

- **schema `v` 不一致時の挙動：Reject（拒否）**

  - schema version field は **top-level `v`**
  - **`v` 未指定は v1 として扱う**（Debug View に警告を出す）
  - サポート外 `v` は **Error View を表示**し、勝手に解釈して描画しない
  - Debug View に `schema v mismatch` を明示する

- **SemVer 方針（Preview）**

  - `laplogos_core` / `laplogos_web` は **常に同一バージョンでロック**
  - Git tag `vX.Y.Z` を **唯一の正**とする
  - `Cargo.toml` / `package.json` が一致しない場合は CI で失敗させる
  - Preview 期間は **0.x 運用**とし、安定後に `1.0.0` へ移行する

### 2) Supported Runtime Baseline（Preview）

- **SSR ランタイム**：Node.js 20.9+ のみ（Edge は対象外）
- **ブラウザ**：Chrome/Edge/Firefox 111+、Safari 16.4+ を最低ライン
- **フレームワーク**：Next.js 前提
- **Next.js**：バージョン下限は指定しない（Preview 期間は最新安定を推奨）

### 3) Distribution（Preview）

- **monorepo のまま運用**（パッケージ公開は Preview 後）
- **wasm 配布**
  - CSR: `/wasm/<file>.wasm` に固定（`public/wasm`）
  - SSR: `<laplogos_web>/public/wasm/<file>.wasm` をデフォルト
  - `LAPLOGOS_WASM_PATH` で上書き可能

### 4) Security Defaults（Capability / fetch）

- `allowed_actions` は **最小セットのみ許可**（必要な action のみ列挙）
- `allowed_hosts` は **デフォルト空（deny）**
- fetch の追加制約（Preview デフォルト）

  - method: **GET のみ**
  - credentials: **omit 固定**
  - headers: **追加禁止（固定）**
  - timeout: **10s**
  - response: **JSON のみ**（`Content-Type: application/json` 必須）
  - max JSON: **1MiB**
  - host は `allowed_hosts` に一致しない場合は必ず Drop（**最終 URL も検査**）

### 5) Observability / Error UX（Preview）

- ログ出力は 3 層で統一する

  1. **Debug View**：State / Action Log / Error Log（常時確認）
  2. **console**：開発者向け詳細
  3. **Copy（再現パッケージ）**：Issue 報告用にワンクリックでコピー可能

- Error View は **最小 UI**で良い（詳細は Debug View に集約）
- 再現パッケージ（最低限含める）

  - schema v / runtime version / page identifier / userAgent
  - input_page（正規化前） + normalized_page（正規化後）
  - state
  - action_log（最新 200 件 or 64KB）

### 6) Performance Guardrails（Preview）

- DoS/暴走防止のため、**core で上限を強制**

  - max nodes: **5000**
  - loop expand: **200**
  - patch/dispatch: **2000**
  - dispatch/sec: **30**

- 計測の出留め

  - Debug View に perf（normalize/materialize/diff/patch/hydrate）を表示
  - console は詳細（必要時）

### 7) Deployment（Preview） & CI Scope

- Preview のデプロイ方式は **Node SSR 1 本に固定**
- CI は **GitHub Actions 新規追加**で OK
- E2E は **Playwright** で OK（主要動線のみ）
- Node 側テストは **node:test（追加依存なし）**で OK

### 8) Documentation（Preview）

- ドキュメント言語：**日本語のみ**
- 配置：README は入口、詳細は `/docs` 配下
- Preview サンプル（最小セット）

  - Counter（set/inc）
  - Form（Inp + Focus Guard）
  - List（Primitive loop + reorder）
  - Router（nav）
  - Fetch（allowed_hosts 前提 + loading）

---

## Phase 13: 互換性ポリシー & リリース形態の確定（Preview 向け）

**目的:** 公開後に「破壊的変更で詰む」事故を避け、Preview としての約束事を固定する。

- [x] **Schema 互換性ポリシー策定**

  - 破壊的変更の定義（必須フィールド追加/意味変更/ID 制約変更 等）
  - `schema v` 不一致時の挙動（拒否・Error View・ログ出力）を仕様として明記

- [x] **SemVer 運用方針（Preview 版）を明記**

  - `laplogos_core` / `laplogos_web` のバージョン付けルール
  - 互換性の保証範囲（Preview では「破壊的変更あり得る」を明示するか等）

- [x] **サポート対象環境の固定**

  - Node バージョン（SSR/テスト/CI で統一）
  - 対象ブラウザ（最低ライン）

- [x] **配布形態の決定**

  - 配布単位（monorepo のまま / package 化 / wasm 配布方法）
  - “Preview の利用方法” を 1 パターンに固定（README に反映）

**Done 条件:** 「互換性」「バージョン」「動作環境」「配布形態」が文章で固定され、変更判断基準がブレない。

---

## Phase 14: セキュリティ最終仕上げ（Preview 必須）

**目的:** “AI 生成データを実行する” 以上、公開前に最低限の守りを固める。

- [x] **脅威モデル（簡易）を作成**

  - 入力点：Schema(JSON)、State、Fetch(URL)、SSR 埋め込み、Router(page)
  - 主要リスク：XSS、任意通信、DoS（巨大ツリー/無限更新）、情報漏えい

- [x] **入力バリデーションの再点検（仕様化）**

  - Global ID Constraint（許可文字種）の適用範囲が全経路で保証されていること
  - Unknown/Invalid Action の Drop ＋ログ（Debug View 含む）方針の固定

- [x] **Capability 運用のデフォルト方針を固定**

  - `allowed_actions` / `allowed_hosts` のデフォルト（基本 deny）
  - Fetch 時の追加制約（必要なら：method/headers/credentials 等の扱いを明記）

- [x] **SSR まわりのセキュリティ仕様の固定**

  - JSON 埋め込みのエスケープ方針（どの文字をどう扱うか）
  - Hydration 失敗時フォールバックの挙動を仕様として明記

- [x] **依存関係・サプライチェーンの最低限チェック方針**

  - Rust / Node の依存をどう点検するか（頻度・CI に入れる/入れない）

**Done 条件:** 「何を守っているか／守れないか」が明文化され、Preview 公開に必要な最低限の安全柵が揃う。

---

## Phase 15: 観測性 & デバッグ体験の運用レベル化

**目的:** 公開後に「壊れた・真っ白」を最短で原因特定できるようにする。

- [x] **ログ設計の確定**

  - warn/error の基準、Action/Schema/Render/Patch のログ粒度
  - Debug View に出す情報（Action Log / State / Schema Tree / Error Log）の確定

- [x] **エラー表示（Error View）方針の確定**

  - Schema 不正、patch 適用失敗、hydration 失敗、fetch 失敗 それぞれの見せ方

- [x] **バグ報告に必要な“再現パッケージ”方針**

  - 最低限：page 識別子、schema v、runtime version、入力 JSON、State、直近 Action ログ
  - 収集手段（UI でコピーできる/ログに出る/ファイルで落とせる等）を決める

- [x] **SSR/CSR の差分調査を容易にする情報表示**

  - どの経路（SSR 生成/CSR hydrate/CSR 更新）で表示されたかが分かるようにする

**Done 条件:** 事故が起きたときに「情報が足りなくて詰む」を回避できる。

---

## Phase 16: パフォーマンス & 制約（Preview の現実ライン確定）

**目的:** “動く” ではなく “使える” を担保し、Preview の上限を宣言できる状態にする。

- [x] **ベンチ・負荷シナリオの定義（Preview 用）**

  - 大きいツリー、長いリスト、頻繁な State 更新、入力中更新、並び替え

- [x] **計測ポイントの確定**

  - normalize/materialize/diff/patch apply/hydration の時間を追える状態にする

- [x] **性能ボトルネックの一次改善**

  - “明らかに重い/不安定” を潰す（フォーカス、reorder、連続 dispatch など）

- [x] **Preview 制約の明文化**

  - 例：最大ノード数、ループの最大要素数、許容更新頻度、禁止パターン
  - DoS 的入力（巨大 JSON 等）の扱い（拒否/制限/警告）を決める

**Done 条件:** Preview としての性能と上限が説明でき、極端入力で破綻しない。

---

## Phase 17: リリース準備の実行（ローカル検証と最終チェック）

**目的:** ベータ公開前の最終手順をローカルで再現し、公開品質を担保する。

- [x] **CI 相当手順のローカル実行**

  - core tests（`laplogos_core`）
  - web lint / node tests（`laplogos_web`）
  - wasm build + 配置（`scripts/prepare-wasm.mjs`）

- [ ] **E2E（Playwright）実行**

  - ローカルでのブラウザ導入と実行結果を確認
  - OS 依存パッケージが不足している場合は導入が必要

- [x] **ドキュメント最終確認**

  - README / preview docs の整合性チェックと必要修正

- [x] **リリースノート & タグ準備**

  - `CHANGELOG.md` に beta リリースノートを追加
  - `vX.Y.Z` の tag 作成準備

- [x] **依存監査（最終）**

  - `cargo audit`
  - `npm audit --production`

**Done 条件:** ローカル実行のログが揃い、ドキュメントとリリースノートが整備されている。

---

## Phase 18: 配布・デプロイの標準化（再現可能なリリース手順）

**目的:** 誰がやっても同じ手順で動き、CI/本番で差異が出ないようにする。

- [x] **ビルド成果物と手順の固定**

  - wasm 生成、web ビルド、SSR での wasm 初期化、CSR での配信経路の標準化

- [x] **環境差分（local/CI/prod）前提の固定**

  - Node runtime 前提の明記（SSR）
  - ファイルパス/アセット配信の前提（デプロイ先でズレない設計にする）

- [x] **CI 上での再現性担保**

  - ビルド → テスト（Phase12）までを CI で一貫実行できる状態
  - 生成物の扱い（キャッシュ/成果物保存の方針）を決める

- [x] **Preview 用デプロイの“1 つの正解”を決める**

  - 実行方法（例：Node SSR で動かす）を一本化
  - 既知の制約（edge 不可等）があれば明記

**Done 条件:** “Preview を配布・起動する手順” が固定され、壊れたら CI で再現できる。

---

## Phase 19: ドキュメント & サンプル（Preview 公開に必要な最低ライン）

**目的:** 第三者（または未来の自分）が迷わず触れ、制約も理解できる状態にする。

- [x] **README（Getting Started）**

  - セットアップ、起動、最小サンプルの動かし方、よくある詰まり

- [x] **Schema v1 仕様書（短縮キー含む）**

  - Node 種別、props、if/for、ID 制約、ActionSpec（set/inc/fetch）、禁止事項（式禁止等）

- [x] **セキュリティ/Capability の説明**

  - allowed_actions/allowed_hosts の思想、デフォルト、運用例

- [x] **SSR/CSR/Hydration の挙動説明**

  - フロー図レベルで OK（SSR 生成 →hydrate→ 更新、失敗時フォールバック）

- [x] **サンプルページ/サンプル JSON**

  - 最小 UI、フォーム入力、ループ、ルーティング、fetch（許可リスト前提）

- [x] **Preview としての制約・既知の非対応**

  - Phase16 で定義した上限、非対応機能、破壊的変更の可能性の明記

**Done 条件:** Preview 公開時に「使い方」「制約」「安全に触る方法」が一式揃う。

---
