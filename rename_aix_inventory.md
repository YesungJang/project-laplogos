# Aix Rename Inventory

## Search 1: Project Aix / AixSchema / Aix / aix_
.gitignore:8:# --- Rust (aix_core) ---
.gitignore:9:/aix_core/target/
.gitignore:10:/aix_core/**/*.rs.bk
.gitignore:13:# --- Node.js / Next.js (aix_web) ---
.gitignore:14:/aix_web/node_modules/
.gitignore:15:/aix_web/.next/
.gitignore:16:/aix_web/out/
.gitignore:17:/aix_web/build/
.gitignore:18:/aix_web/.env
.gitignore:19:/aix_web/.env.local
.gitignore:20:/aix_web/.env.development.local
.gitignore:21:/aix_web/.env.test.local
.gitignore:22:/aix_web/.env.production.local
.gitignore:28:# --- Project Aix Specific ---
DEV_PLAN.md:1:# Project Aix: Master Development Plan (dev_plan.md)
DEV_PLAN.md:6:- **Core:** `aix_core` (Rust / WebAssembly)
DEV_PLAN.md:7:- **Web:** `aix_web` (Next.js / TypeScript)
DEV_PLAN.md:29:- MVP では `AixPage.root` は **常に 1 ノード**（Materialize 後も 1 ノード）
DEV_PLAN.md:40:- [x] Task 1.2: Schema v1 Definition (`AixPage`, `AixNode`).
DEV_PLAN.md:48:- [x] Task 2.3: Custom Hook (`useAixEngine`).
DEV_PLAN.md:53:- [x] Task 3.1: Store & Action Struct (`AixState`).
DEV_PLAN.md:54:- [x] Task 3.2: Engine Struct (`AixEngine`).
DEV_PLAN.md:73:## Phase 5: Restricted Dynamic Binding (`aix_core`)
DEV_PLAN.md:82:- [x] **Target:** `aix_core/src/store.rs` & `aix_core/src/lib.rs`
DEV_PLAN.md:116:- [x] **Target:** `aix_core/src/materialize.rs`（Task 5.5 で新設）または `renderer.rs`
DEV_PLAN.md:121:- `fn resolve_text(val: &Value, state: &AixState, scope: &MaterializeScope) -> String`
DEV_PLAN.md:135:- [x] **Target:** `aix_core/src/materialize.rs`（推奨）または `renderer.rs`
DEV_PLAN.md:145:- [x] **Target:** `aix_core/src/materialize.rs`（推奨）+ `tests/conformance.rs`
DEV_PLAN.md:150:- `AixNode.i` は常に **`[A-Za-z0-9_:-]+`** を満たすこと
DEV_PLAN.md:159:- [x] **Target:** `aix_core/src/renderer.rs`
DEV_PLAN.md:175:- [x] **Target:** `aix_core/src/materialize.rs`（新規）
DEV_PLAN.md:187:- `fn materialize_many(node: &AixNode, state: &AixState, scope: &MaterializeScope) -> Result<Vec<AixNode>, AixError>`
DEV_PLAN.md:191:- `fn materialize_one_root(root: &AixNode, state: &AixState) -> Result<AixNode, AixError>`
DEV_PLAN.md:211:## Phase 6: Client-Side Smart Diffing (`aix_core`)
DEV_PLAN.md:221:- [x] **Target:** `aix_core/src/lib.rs`
DEV_PLAN.md:223:- `AixEngine` は `last_materialized: Option<AixNode>` を保持する
DEV_PLAN.md:234:- [x] **Target:** `aix_core/src/diff.rs`（新規）
DEV_PLAN.md:239:- `Append { parent_id: String, node: AixNode }`
DEV_PLAN.md:241:- `Replace { id: String, node: AixNode }`
DEV_PLAN.md:248:- `diff(old: &AixNode, new: &AixNode) -> Vec<Patch>` は **materialized tree 同士**で比較する
DEV_PLAN.md:264:- [x] **Target:** `aix_core/src/renderer.rs`
DEV_PLAN.md:285:- [x] **Target:** `aix_core/src/renderer.rs`
DEV_PLAN.md:293:## Phase 7: Capability & Security (`aix_core`)
DEV_PLAN.md:302:- [x] **Target:** `aix_core/src/store.rs` & `aix_core/src/lib.rs`
DEV_PLAN.md:304:- `struct AixConfig { allowed_actions: Vec<String>, allowed_hosts: Vec<String> }`
DEV_PLAN.md:305:- `AixEngine::init(config)` で注入可能
DEV_PLAN.md:313:- [x] **Target:** `aix_core/src/lib.rs`
DEV_PLAN.md:331:## Phase 8: Reliability & Testing (`aix_core`)
README.md:1:# Project Aix (MVP)
README.md:16:- **Schema:** Aix Schema v1 (JSON based)
README.md:20:- `aix_core/`: Rust 製ランタイムエンジン (Wasm)
README.md:21:- `aix_web/`: フロントエンド (Next.js) + デバッグ可視化ツール
README.md:35:   cd aix_core
aix_core/Cargo.toml:2:name = "aix_core"
aix_core/src/diff.rs:3:use crate::AixNode;
aix_core/src/diff.rs:7:    Append { parent_id: String, node: AixNode },
aix_core/src/diff.rs:9:    Replace { id: String, node: AixNode },
aix_core/src/diff.rs:15:pub fn diff(old: &AixNode, new: &AixNode) -> Vec<Patch> {
aix_core/src/diff.rs:40:fn diff_props(old: &AixNode, new: &AixNode, patches: &mut Vec<Patch>) {
aix_core/src/diff.rs:70:fn diff_children(old: &AixNode, new: &AixNode, patches: &mut Vec<Patch>) {
aix_core/src/diff.rs:74:    let old_map: HashMap<String, AixNode> =
aix_core/src/diff.rs:76:    let new_map: HashMap<String, AixNode> =
aix_core/src/lib.rs:16:pub use store::AixState;
aix_core/src/lib.rs:17:use store::{apply_action, normalize_action, validate_action, AixConfig, ValidatedAction};
aix_core/src/lib.rs:18:use crate::materialize::{materialize_one_root, AixError};
aix_core/src/lib.rs:21:pub struct AixPage {
aix_core/src/lib.rs:23:    pub root: AixNode,
aix_core/src/lib.rs:27:pub struct AixNode {
aix_core/src/lib.rs:35:    pub c: Option<Vec<AixNode>>,
aix_core/src/lib.rs:45:pub struct AixEngine {
aix_core/src/lib.rs:46:    inner: Rc<RefCell<AixEngineInner>>,
aix_core/src/lib.rs:49:struct AixEngineInner {
aix_core/src/lib.rs:50:    state: AixState,
aix_core/src/lib.rs:51:    page: AixPage,
aix_core/src/lib.rs:53:    last_materialized: Option<AixNode>,
aix_core/src/lib.rs:54:    config: Option<AixConfig>,
aix_core/src/lib.rs:57:impl Clone for AixEngine {
aix_core/src/lib.rs:59:        AixEngine {
aix_core/src/lib.rs:66:impl AixEngine {
aix_core/src/lib.rs:68:    pub fn new(initial_json: &str) -> Result<AixEngine, JsValue> {
aix_core/src/lib.rs:69:        let page: AixPage = serde_json::from_str(initial_json)
aix_core/src/lib.rs:72:        Ok(AixEngine {
aix_core/src/lib.rs:73:            inner: Rc::new(RefCell::new(AixEngineInner {
aix_core/src/lib.rs:74:                state: AixState::default(),
aix_core/src/lib.rs:89:        let mut config: AixConfig = serde_json::from_str(config_json)
aix_core/src/lib.rs:114:impl AixEngine {
aix_core/src/lib.rs:120:    fn with_page<R>(&self, f: impl FnOnce(&AixPage) -> R) -> R {
aix_core/src/lib.rs:191:    pub(crate) fn page_root(&self) -> AixNode {
aix_core/src/lib.rs:216:    fn materialize_current(&self) -> Result<(AixNode, Vec<String>), JsValue> {
aix_core/src/lib.rs:231:    fn log_error(&self, err: &AixError) {
aix_core/src/lib.rs:313:    let page: AixPage =
aix_core/src/materialize.rs:4:use crate::{store::AixState, AixNode};
aix_core/src/materialize.rs:12:pub fn resolve_text(val: &Value, state: &AixState, scope: &MaterializeScope) -> String {
aix_core/src/materialize.rs:42:pub fn resolve_if(val: &Value, state: &AixState) -> (bool, Option<String>) {
aix_core/src/materialize.rs:58:pub struct AixError {
aix_core/src/materialize.rs:62:impl AixError {
aix_core/src/materialize.rs:78:pub fn validate_global_ids(node: &AixNode) -> Result<(), String> {
aix_core/src/materialize.rs:91:    node: &AixNode,
aix_core/src/materialize.rs:92:    state: &AixState,
aix_core/src/materialize.rs:95:) -> Result<Vec<AixNode>, AixError> {
aix_core/src/materialize.rs:97:        return Err(AixError::new(format!("Invalid id: {}", node.i)));
aix_core/src/materialize.rs:113:            _ => return Err(AixError::new("for must be an array")),
aix_core/src/materialize.rs:125:                        return Err(AixError::new("for array must be all strings or all numbers"));
aix_core/src/materialize.rs:132:                        return Err(AixError::new("for array must be all strings or all numbers"));
aix_core/src/materialize.rs:136:                _ => return Err(AixError::new("for array items must be string or number")),
aix_core/src/materialize.rs:140:                return Err(AixError::new(format!("Invalid loop item id: {}", item_str)));
aix_core/src/materialize.rs:143:                return Err(AixError::new(format!("Duplicate loop item: {}", item_str)));
aix_core/src/materialize.rs:156:                return Err(AixError::new(format!("Invalid generated id: {}", node_copy.i)));
aix_core/src/materialize.rs:176:    root: &AixNode,
aix_core/src/materialize.rs:177:    state: &AixState,
aix_core/src/materialize.rs:179:) -> Result<AixNode, AixError> {
aix_core/src/materialize.rs:181:        return Err(AixError::new("root cannot have for"));
aix_core/src/materialize.rs:187:        0 => Err(AixError::new("root if=false is fatal")),
aix_core/src/materialize.rs:188:        _ => Err(AixError::new("root must materialize to a single node")),
aix_core/src/materialize.rs:192:fn apply_text(node: &mut AixNode, state: &AixState, scope: &MaterializeScope) {
aix_core/src/materialize.rs:220:    node: &mut AixNode,
aix_core/src/materialize.rs:221:    state: &AixState,
aix_core/src/materialize.rs:224:) -> Result<(), AixError> {
aix_core/src/renderer.rs:4:use crate::{diff::Patch, AixEngine, AixNode};
aix_core/src/renderer.rs:12:    node: &AixNode,
aix_core/src/renderer.rs:13:    engine: Option<&AixEngine>,
aix_core/src/renderer.rs:84:pub fn update_dom(engine: &AixEngine, container_id: &str) -> Result<(), JsValue> {
aix_core/src/renderer.rs:102:    root: &AixNode,
aix_core/src/renderer.rs:103:    engine: Option<&AixEngine>,
aix_core/src/renderer.rs:124:    engine: &AixEngine,
aix_core/src/renderer.rs:150:    engine: &AixEngine,
aix_core/src/renderer.rs:153:    new_root: &AixNode,
aix_core/src/renderer.rs:218:pub fn render_to_string(node: &AixNode) -> String {
aix_core/src/store.rs:6:pub struct AixState {
aix_core/src/store.rs:11:pub struct AixConfig {
aix_core/src/store.rs:18:impl AixConfig {
aix_core/src/store.rs:157:    state: &mut AixState,
aix_core/tests/conformance.rs:4:use aix_core::{materialize::materialize_one_root, AixPage, AixState};
aix_core/tests/conformance.rs:7:fn load_page(path: &Path) -> AixPage {
aix_core/tests/conformance.rs:12:fn state_for(path: &Path) -> AixState {
aix_core/tests/conformance.rs:13:    let mut state = AixState::default();
aix_core/tests/snapshot.rs:1:use aix_core::{materialize::materialize_one_root, render_to_string, AixPage, AixState};
aix_core/tests/snapshot.rs:6:    let page: AixPage = serde_json::from_str(
aix_core/tests/snapshot.rs:10:    let mut state = AixState::default();
aix_web/app/page.tsx:5:import { useAixEngine } from "../hooks/useAixEngine";
aix_web/app/page.tsx:28:  const { engine, dispatch, refreshTrigger } = useAixEngine(initialJson, containerId);
aix_web/components/DebugView.tsx:5:type Engine = Awaited<ReturnType<typeof import("aix_core")>>["AixEngine"];
aix_web/hooks/useAixEngine.ts:3:type EngineModule = typeof import("aix_core");
aix_web/hooks/useAixEngine.ts:4:type Engine = InstanceType<EngineModule["AixEngine"]>;
aix_web/hooks/useAixEngine.ts:6:type UseAixEngineResult = {
aix_web/hooks/useAixEngine.ts:12:export function useAixEngine(initialJson: string, containerId: string): UseAixEngineResult {
aix_web/hooks/useAixEngine.ts:21:        const mod = await import("aix_core");
aix_web/hooks/useAixEngine.ts:28:        const inst = new mod.AixEngine(initialJson);
aix_web/next.config.ts:7:  transpilePackages: ["aix_core"],
aix_web/package-lock.json:2:  "name": "aix_web",
aix_web/package-lock.json:8:      "name": "aix_web",
aix_web/package-lock.json:11:        "aix_core": "file:../aix_core/pkg",
aix_web/package-lock.json:27:    "../aix_core/pkg": {
aix_web/package-lock.json:28:      "name": "aix_core",
aix_web/package-lock.json:2144:    "node_modules/aix_core": {
aix_web/package-lock.json:2145:      "resolved": "../aix_core/pkg",
aix_web/package.json:2:  "name": "aix_web",
aix_web/package.json:12:    "aix_core": "file:../aix_core/pkg",
codex-history.md:5:2025-12-18 23:46 cargo checkエラーを修正(web-sysにversion追加・AixPageにderive追加)
codex-history.md:8:2025-12-18 23:52 タスク2.3 useAixEngineフックを作成
codex-history.md:11:2025-12-19 00:00 useAixEngineのpkg importパスを修正
codex-history.md:13:2025-12-19 00:07 Turbopack向けにaix_coreをfile依存として追加しimportをパッケージ参照に変更
codex-history.md:14:2025-12-19 00:10 Turbopack用にaix_coreへのresolveAliasとtranspile設定を追加
codex-history.md:16:2025-12-19 00:15 useAixEngineのインポートを相対パスに戻しturbopack設定最小化で対応
codex-history.md:17:2025-12-19 00:24 webpackフラグでdevを固定しaix_coreをパッケージ名インポートに戻して再構成
codex-history.md:19:2025-12-19 00:32 useAixEngineでWasm初期化後にinit_panic_hookを呼ぶよう修正
codex-history.md:20:2025-12-20 11:55 タスク3.1 store.rsにAixState/Action/reduceを実装
codex-history.md:21:2025-12-20 11:56 タスク3.2 AixEngine構造体をlib.rsに追加し状態保持を実装
codex-history.md:25:2025-12-20 12:04 タスク4.2 useAixEngineをAixEngine生成・dispatchラッパー付きに拡張
codex-history.md:28:2025-12-20 12:23 wasm-packを再ビルドしてAixEngineをpkgに含めた

## Search 2: aix_core / aix_web / render_aix / useAixEngine
.gitignore:8:# --- Rust (aix_core) ---
.gitignore:9:/aix_core/target/
.gitignore:10:/aix_core/**/*.rs.bk
.gitignore:13:# --- Node.js / Next.js (aix_web) ---
.gitignore:14:/aix_web/node_modules/
.gitignore:15:/aix_web/.next/
.gitignore:16:/aix_web/out/
.gitignore:17:/aix_web/build/
.gitignore:18:/aix_web/.env
.gitignore:19:/aix_web/.env.local
.gitignore:20:/aix_web/.env.development.local
.gitignore:21:/aix_web/.env.test.local
.gitignore:22:/aix_web/.env.production.local
DEV_PLAN.md:6:- **Core:** `aix_core` (Rust / WebAssembly)
DEV_PLAN.md:7:- **Web:** `aix_web` (Next.js / TypeScript)
DEV_PLAN.md:42:- [x] Task 1.4: Entry Point (`render_aix`).
DEV_PLAN.md:48:- [x] Task 2.3: Custom Hook (`useAixEngine`).
DEV_PLAN.md:73:## Phase 5: Restricted Dynamic Binding (`aix_core`)
DEV_PLAN.md:82:- [x] **Target:** `aix_core/src/store.rs` & `aix_core/src/lib.rs`
DEV_PLAN.md:116:- [x] **Target:** `aix_core/src/materialize.rs`（Task 5.5 で新設）または `renderer.rs`
DEV_PLAN.md:135:- [x] **Target:** `aix_core/src/materialize.rs`（推奨）または `renderer.rs`
DEV_PLAN.md:145:- [x] **Target:** `aix_core/src/materialize.rs`（推奨）+ `tests/conformance.rs`
DEV_PLAN.md:159:- [x] **Target:** `aix_core/src/renderer.rs`
DEV_PLAN.md:175:- [x] **Target:** `aix_core/src/materialize.rs`（新規）
DEV_PLAN.md:211:## Phase 6: Client-Side Smart Diffing (`aix_core`)
DEV_PLAN.md:221:- [x] **Target:** `aix_core/src/lib.rs`
DEV_PLAN.md:234:- [x] **Target:** `aix_core/src/diff.rs`（新規）
DEV_PLAN.md:264:- [x] **Target:** `aix_core/src/renderer.rs`
DEV_PLAN.md:285:- [x] **Target:** `aix_core/src/renderer.rs`
DEV_PLAN.md:293:## Phase 7: Capability & Security (`aix_core`)
DEV_PLAN.md:302:- [x] **Target:** `aix_core/src/store.rs` & `aix_core/src/lib.rs`
DEV_PLAN.md:313:- [x] **Target:** `aix_core/src/lib.rs`
DEV_PLAN.md:331:## Phase 8: Reliability & Testing (`aix_core`)
README.md:20:- `aix_core/`: Rust 製ランタイムエンジン (Wasm)
README.md:21:- `aix_web/`: フロントエンド (Next.js) + デバッグ可視化ツール
README.md:35:   cd aix_core
aix_core/Cargo.toml:2:name = "aix_core"
aix_core/src/lib.rs:312:pub fn render_aix(json_str: &str, container_id: &str) -> Result<(), JsValue> {
aix_core/tests/conformance.rs:4:use aix_core::{materialize::materialize_one_root, AixPage, AixState};
aix_core/tests/snapshot.rs:1:use aix_core::{materialize::materialize_one_root, render_to_string, AixPage, AixState};
aix_web/app/page.tsx:5:import { useAixEngine } from "../hooks/useAixEngine";
aix_web/app/page.tsx:28:  const { engine, dispatch, refreshTrigger } = useAixEngine(initialJson, containerId);
aix_web/components/DebugView.tsx:5:type Engine = Awaited<ReturnType<typeof import("aix_core")>>["AixEngine"];
aix_web/hooks/useAixEngine.ts:3:type EngineModule = typeof import("aix_core");
aix_web/hooks/useAixEngine.ts:12:export function useAixEngine(initialJson: string, containerId: string): UseAixEngineResult {
aix_web/hooks/useAixEngine.ts:21:        const mod = await import("aix_core");
aix_web/next.config.ts:7:  transpilePackages: ["aix_core"],
aix_web/package-lock.json:2:  "name": "aix_web",
aix_web/package-lock.json:8:      "name": "aix_web",
aix_web/package-lock.json:11:        "aix_core": "file:../aix_core/pkg",
aix_web/package-lock.json:27:    "../aix_core/pkg": {
aix_web/package-lock.json:28:      "name": "aix_core",
aix_web/package-lock.json:2144:    "node_modules/aix_core": {
aix_web/package-lock.json:2145:      "resolved": "../aix_core/pkg",
aix_web/package.json:2:  "name": "aix_web",
aix_web/package.json:12:    "aix_core": "file:../aix_core/pkg",
codex-history.md:8:2025-12-18 23:52 タスク2.3 useAixEngineフックを作成
codex-history.md:9:2025-12-18 23:53 タスク2.4 page.tsxでテストJSONをrender_aixする実装に変更
codex-history.md:11:2025-12-19 00:00 useAixEngineのpkg importパスを修正
codex-history.md:13:2025-12-19 00:07 Turbopack向けにaix_coreをfile依存として追加しimportをパッケージ参照に変更
codex-history.md:14:2025-12-19 00:10 Turbopack用にaix_coreへのresolveAliasとtranspile設定を追加
codex-history.md:16:2025-12-19 00:15 useAixEngineのインポートを相対パスに戻しturbopack設定最小化で対応
codex-history.md:17:2025-12-19 00:24 webpackフラグでdevを固定しaix_coreをパッケージ名インポートに戻して再構成
codex-history.md:19:2025-12-19 00:32 useAixEngineでWasm初期化後にinit_panic_hookを呼ぶよう修正
codex-history.md:25:2025-12-20 12:04 タスク4.2 useAixEngineをAixEngine生成・dispatchラッパー付きに拡張
