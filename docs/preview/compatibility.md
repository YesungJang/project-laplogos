# Preview Compatibility & Versioning

## Schema compatibility

- schema version field: top-level `v`
- `v` 未指定は v1 として扱い、Debug View に警告を出す
- サポート外 `v` は Error View（SSR/CSR ともに描画しない）

### 破壊的変更（`v` を上げる）

- 必須フィールド追加 / 型変更 / 意味変更
- ID 制約（許可文字種）変更
- 同一入力で DOM 結果が変わる Diff/Patch 仕様変更

### 破壊的でない変更（`v` 据え置き）

- Node type 追加 / 任意 props 追加 / Debug 情報追加

## SemVer（Preview）

- `laplogos_core` / `laplogos_web` は常に同一バージョン
- Git tag `vX.Y.Z` を唯一の正とし、CI で整合性を検証
- Preview 期間は `0.x` 運用、安定後に `1.0.0`

## Supported runtime

- SSR: Node.js 20.9+
- Browser: Chrome/Edge/Firefox 111+、Safari 16.4+
- Framework: Next.js（バージョン下限は指定しない）

## Distribution

- monorepo 運用（Preview 後にパッケージ公開を検討）
- CSR wasm: `/wasm/<file>.wasm`（`laplogos_web/public/wasm`）
- SSR wasm: `laplogos_web/public/wasm/<file>.wasm` がデフォルト
- `LAPLOGOS_WASM_PATH` で SSR の wasm パスを上書き可能
