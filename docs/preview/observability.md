# Preview Observability

## Log design

- **warn**: 非致命（Action Drop / fetch 失敗 / hydration 失敗 / patch 失敗）
- **error**: 致命（schema 不正 / guardrail 超過 / 互換性違反）
- Debug View は **State / Action Log / Schema Tree / Error Log / Perf** を表示

## Error View policy

- schema 不正 / schema v 不一致 / guardrail 超過 → Error View
- patch 適用失敗 → warn + root 置換（Error View は出さない）
- hydration 失敗 → warn + root 置換（Error View は出さない）
- fetch 失敗 → warn + on_fail dispatch（Error View は出さない）

### Error View の表示仕様

- 外側のコンテナに `role="alert"` を付与し、支援技術へエラーを通知する
- メッセージはプレーンテキストとして表示する（HTML として解釈せず、切り詰めや整形もしない。
  CSR は React の子要素として、SSR は `<` / `>` / `&` / `"` をエスケープして出力）
- 復旧手順を 1 文で表示する：「ページを再読み込みし、解決しない場合は Debug View を確認してください。」
- SSR（`lib/laplogos-ssr.ts` の `renderErrorHtml`）と CSR（`components/ErrorView.tsx`）は
  同じ構造（role / 見出し / メッセージ要素 / 復旧手順 / クラス）を出力し、hydration で不一致にならない

## 再現パッケージ

- Debug View から Copy/Download
- 含めるもの:
  - schema v / runtime version / page identifier / userAgent
  - input_page（正規化前） / normalized_page（正規化後）
  - state
  - action_log（最新 200 件 or 64KB）
  - error_log

## SSR/CSR 判定表示

- Debug View に `SSR+Hydration` / `CSR` / `CSR Update` を表示
