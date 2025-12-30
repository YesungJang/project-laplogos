# Preview Security Defaults

## Threat model（簡易）

入力点:

- Schema（JSON）
- State
- fetch（URL）
- SSR 埋め込み JSON
- Router（page）

主要リスク:

- XSS（埋め込み/レンダリング）
- 任意通信（fetch/外部遷移）
- DoS（巨大ツリー/連続 dispatch）
- 情報漏えい（ログ/再現パッケージ）

## 入力バリデーション

- Global ID Constraint は全レンダリング経路で必須
- Unknown/Invalid Action は Drop + warn/error log
- ループ/if 制約は materialize 時に厳格チェック

## Capability defaults

- `allowed_actions` は **必要最小限を列挙**
- `allowed_hosts` は **デフォルト空（deny）**
- クライアント設定は `public/laplogos-config.json` または環境変数で注入する
  - `NEXT_PUBLIC_LAPLOGOS_ALLOWED_ACTIONS`
  - `NEXT_PUBLIC_LAPLOGOS_ALLOWED_HOSTS`

## fetch defaults（Preview）

- method: GET のみ
- credentials: omit 固定
- headers: 追加禁止
- timeout: 10s
- response: `Content-Type: application/json` 必須
- max JSON: 1MiB
- host は `allowed_hosts` でチェック（**最終 URL も検査**）

## SSR security

- 埋め込み JSON は `<` を `\\u003c` に置換して XSS 回避
- Hydration 失敗時は root 置換へフォールバック

## 依存関係チェック方針

- Preview 期間は **main/リリース前に監査**（`cargo audit`, `npm audit --production`）
- PR では必須化しない（スピード優先）
