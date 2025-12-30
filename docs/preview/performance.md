# Preview Performance & Guardrails

## ベンチ/負荷シナリオ（Preview）

- 大きいツリー（深さ/幅の両方）
- 長いリスト（for 展開）
- 頻繁な state 更新
- 入力中の更新（Focus Guard）
- 並び替え（ReorderChildren）

## 計測ポイント

- normalize
- materialize
- diff
- patch apply
- hydration

## 既存の性能対策

- Focus Guard（入力中の value 上書きを回避）
- ReorderChildren パッチで DOM 破壊を最小化

## Guardrails（core 強制）

- max nodes: 5000（超過は Fatal）
- max depth: 256（超過は Fatal）
- loop expand: 200（超過は Fatal）
- patch/dispatch: 2000（超過は Fatal）
- dispatch/sec: 30（超過は Drop + warn）

## Preview 制約

- 式評価は行わない（`$key` の完全一致のみ）
- root 直下 for は禁止
- fetch は JSON のみ（1MiB 上限）
