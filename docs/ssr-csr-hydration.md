# SSR / CSR / Hydration

## SSR

- `render_laplogos_to_string` で HTML を生成
- 初期データは `<script type="application/json">` に埋め込み
- `<` は `\\u003c` に置換して XSS を回避
- ルート要素に `data-laplogos-root="1"` を付与

## Hydration

- bootstrap JSON を読み取り、Normalize → Materialize
- `bind_events` でイベントのみ接続
- 失敗時は root 置換でフル再描画（root マーカーも付与）

## CSR update

- ルート変更時に schema を再取得
- Normalize → Materialize → Diff → Patch

## Error 表示（SSR / CSR 共通）

CSR は `components/ErrorView.tsx`、SSR は `lib/laplogos-ssr.ts` の `renderErrorHtml` が
同じエラー表示を描画します。文言・クラス・メッセージの正規化は
`lib/laplogos-error-view.ts` に集約し、両経路で同じ内容を保ちます。

- コンテナに `role="alert"` / `aria-live="assertive"` / `aria-atomic="true"` を付与し、
  エラーが出たときに支援技術が読み上げます
- メッセージは常に**プレーンテキスト**として表示します。HTML として解釈せず、
  SSR 側は `&` / `<` / `>` / `"` をエスケープします（`<script>` などは文字列のまま出ます）
- メッセージの下に復旧手順
  （`ページを再読み込みするか、schema を修正して再試行してください。`）と
  Debug View への案内を並べて表示します
- メッセージが文字列でない場合や空白のみの場合は、`undefined` や `[object Object]` を
  出さずに固定の代替文（`不明なエラーが発生しました。`）を表示します
- `ErrorView` の props は `message: string` のままで、既存の呼び出しは変更不要です

どのケースで Error View を出すかの方針は `docs/preview/observability.md` を参照してください。
