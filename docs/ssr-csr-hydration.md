# SSR / CSR / Hydration

## SSR

- `render_laplogos_to_string` で HTML を生成
- 初期データは `<script type="application/json">` に埋め込み
- `<` は `\\u003c` に置換して XSS を回避

## Hydration

- bootstrap JSON を読み取り、Normalize → Materialize
- `bind_events` でイベントのみ接続
- 失敗時は root 置換でフル再描画

## CSR update

- ルート変更時に schema を再取得
- Normalize → Materialize → Diff → Patch
