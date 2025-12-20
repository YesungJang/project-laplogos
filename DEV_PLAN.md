# Project Aix: Master Development Plan (dev_plan.md)

**Project Goal:** "Human-Unreadable, Machine-Optimal"  
AI ネイティブ時代の次世代 UI ランタイム。Web での完成度（Stage 2）を高めてから Native 展開（Stage 3）へ進む。

- **Core:** `aix_core` (Rust / WebAssembly)
- **Web:** `aix_web` (Next.js / TypeScript)
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

- MVP では `AixPage.root` は **常に 1 ノード**（Materialize 後も 1 ノード）
  - root 直下で `for` を許可しない（必要になったら v2 で検討）
  - root に `if` を付けるのは可。ただし `if=false` の場合は **Fatal**（空画面ではなく Error View）

---

## ✅ Stage 1: Foundation (Web MVP) [COMPLETED]

### Phase 1: Rust Core Foundation

- [x] Task 1.1: Dependencies (`serde`, `wasm-bindgen`, `web-sys`) setup.
- [x] Task 1.2: Schema v1 Definition (`AixPage`, `AixNode`).
- [x] Task 1.3: Basic Renderer (`Box`, `Txt`, `Btn`).
- [x] Task 1.4: Entry Point (`render_aix`).

### Phase 2: Next.js Integration

- [x] Task 2.1: Wasm Build setup (`wasm-pack`).
- [x] Task 2.2: Next.js Config (`asyncWebAssembly`).
- [x] Task 2.3: Custom Hook (`useAixEngine`).
- [x] Task 2.4: Screen Display (Hello AIX).

### Phase 3: State & Action Logic

- [x] Task 3.1: Store & Action Struct (`AixState`).
- [x] Task 3.2: Engine Struct (`AixEngine`).
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

## Phase 5: Restricted Dynamic Binding (`aix_core`)

**方針:**  
式評価を徹底排除し、構造化された Action 定義と厳格なループ制約により、ランタイムの肥大化とバグを防ぐ。

---

### Task 5.0: Action Normalization (ActionSpec)

- [x] **Target:** `aix_core/src/store.rs` & `aix_core/src/lib.rs`
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

- [x] **Target:** `aix_core/src/materialize.rs`（Task 5.5 で新設）または `renderer.rs`
- [ ] **Spec:**

**Helper:**

- `fn resolve_text(val: &Value, state: &AixState, scope: &MaterializeScope) -> String`
- 文字列かつ `"$key"` (完全一致) の場合のみ State から取得
- 部分一致 (`"Hi $name"`) / `${}` / 演算は **非対応**（そのまま表示）

**Scope（置換対象）:**

- `Txt.p["val"]`
- `Btn.p["label"]`
- `Inp.p["val"]`

---

### Task 5.2: Strict Boolean If

- [x] **Target:** `aix_core/src/materialize.rs`（推奨）または `renderer.rs`
- [ ] **Spec:**
- **Logic:** State 値が **Boolean(true)** の場合のみ通す
- `"true"`(文字列), `1`, `null`, 未定義キーは全て false 扱い
- `if` 判定が非 bool だった場合は warn を残す（ただし挙動は false）

---

### Task 5.3: Global ID Constraint

- [x] **Target:** `aix_core/src/materialize.rs`（推奨）+ `tests/conformance.rs`
- [ ] **Spec:**

**Global Schema Constraint（Fatal）:**

- `AixNode.i` は常に **`[A-Za-z0-9_:-]+`** を満たすこと
- 違反が 1 つでもあれば **Fatal**（Error View + log、DOM 更新停止）

> NOTE: `:` は `{parent}:{item}` のために許可する。

---

### Task 5.4: Action-based Input (No Magic Binding)

- [x] **Target:** `aix_core/src/renderer.rs`
- [ ] **Spec:**
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

- [ ] **Target:** `aix_core/src/materialize.rs`（新規）
- [ ] **Spec:**

#### Data Structures

- `struct MaterializeScope {`
  - `item: Option<String>` // "$item" 用（ループ内のみ）
  - `key: Option<String>` // "$key" 用（ループ内のみ）
  - `}`

#### Functions (重要: “複数ノードを返せる” 形にする)

- `fn materialize_many(node: &AixNode, state: &AixState, scope: &MaterializeScope) -> Result<Vec<AixNode>, AixError>`
  - 通常ノード: `Ok(vec![one])`
  - `if=false`: `Ok(vec![])`（ノードを消す）
  - `for`: `Ok(vec![expanded...])`（複数ノード展開）
- `fn materialize_one_root(root: &AixNode, state: &AixState) -> Result<AixNode, AixError>`
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

## Phase 6: Client-Side Smart Diffing (`aix_core`)

**方針:**  
DOM の破壊的更新を最小限にし、フォーカス維持とパフォーマンス確保を行う。  
Diff 対象は必ず **Materialized Tree**。

---

### Task 6.0: Engine Loop Update (Materialize → Diff → Patch)

- [ ] **Target:** `aix_core/src/lib.rs`
- [ ] **Spec:**
- `AixEngine` は `last_materialized: Option<AixNode>` を保持する
- `dispatch` 後のレンダリングフロー:
  1. `new_materialized = materialize_one_root(page.root, state)`
  2. `patches = diff(last_materialized, new_materialized)`
  3. `apply_patches(patches)`（失敗時は fallback）
  4. `last_materialized = new_materialized`

---

### Task 6.1: Diff Engine & Patch Definition

- [ ] **Target:** `aix_core/src/diff.rs`（新規）
- [ ] **Spec:**

**Enum `Patch`:**

- `Append { parent_id: String, node: AixNode }`
- `Remove { id: String }`
- `Replace { id: String, node: AixNode }`
- `UpdateText { id: String, val: String }`
- `UpdateAttr { id: String, key: String, val: String }`
- `ReorderChildren { parent_id: String, ordered_ids: Vec<String> }`

**Diff 対象:**

- `diff(old: &AixNode, new: &AixNode) -> Vec<Patch>` は **materialized tree 同士**で比較する

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

- [ ] **Target:** `aix_core/src/renderer.rs`
- [ ] **Spec:**

**Preparation:**

- `render_node` は全要素に `data-aix-id="{node.i}"` を付与する
- 可能ならルート要素にも `data-aix-root="1"` を付与（fallback 置換を容易にする）

**Lookup:**

- `document.querySelector(&format!("[data-aix-id=\"{}\"]", id))` を使う
- `id` は Global ID Constraint により selector-safe

**Fallback Policy:**

- 対象 DOM が見つからない / 操作失敗 → Error Log 記録 → **フル再描画で root を置換**（破綻状態を回避）

---

### Task 6.3: Focus Guard

- [ ] **Target:** `aix_core/src/renderer.rs`
- [ ] **Spec:**
- `UpdateAttr` で `value` 更新する際、
  - 対象要素が `document.activeElement` と一致する場合は **スキップ**
- 目的: 入力中の value 上書き/フォーカス喪失を防ぐ

---

## Phase 7: Capability & Security (`aix_core`)

**方針:**  
セキュリティ基盤を先に整え、その上で通信機能を実装する。

---

### Task 7.1: Capability System

- [ ] **Target:** `aix_core/src/store.rs` & `aix_core/src/lib.rs`
- [ ] **Spec:**
- `struct AixConfig { allowed_actions: Vec<String>, allowed_hosts: Vec<String> }`
- `AixEngine::init(config)` で注入可能
- `dispatch` 直前で `allowed_actions` をチェック（未許可は Drop + log）
- URL チェックヘルパー（`allowed_hosts` と照合）を実装

---

### Task 7.2: Async Fetch Action

- [ ] **Target:** `aix_core/src/lib.rs`
- [ ] **Spec:**

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

## Phase 8: Reliability & Testing (`aix_core`)

**方針:**  
仕様準拠を保証するテスト基盤を構築（クラッシュしない / 仕様がブレない）。

---

### Task 8.1: Conformance Suite

- [ ] **Target:** `tests/conformance.rs`
- [ ] **Spec:**
- `tests/data/` に `valid_*.json` / `invalid_*.json` を配置
- 一括テストランナーを作成
  - Valid は成功
  - Invalid（ID 文字種違反・loop 重複・loop 型混在・if 非 bool など）は **Fatal として Error になる**こと
  - panic しないこと

---

### Task 8.2: Deterministic Render Test (Snapshot)

- [ ] **Target:** `tests/snapshot.rs`
- [ ] **Spec:**
- DOM を触らずに比較できる形にする
  - `materialize_one_root` → `render_to_string(materialized_root)` の純関数パス
- 「特定の State + JSON」で常に同じ HTML が生成されることを検証

---

## 🧱 Stage 2.5: Web Production Readiness

- Phase 9: Token Optimization（Def/Ref, Style Dict）
- Phase 10: Router / SPA
- Phase 11: SSR & Hydration

---

## 📱 Stage 3: Multi-Platform (Native)

- Phase 12: iOS Runtime（SwiftUI）
- Phase 13: Android Runtime（Jetpack Compose）

---

## 🤖 Stage 4: AI-Native Evolution

- Phase 14: Streaming UI
- Phase 15: Self-Healing / A/B Testing
