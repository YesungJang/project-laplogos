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
