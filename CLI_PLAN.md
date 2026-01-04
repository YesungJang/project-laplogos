# laplogos CLI 計画書（ドラフト）

## 目的
- 開発者が「最短で試せる」体験を提供する
- schema 作成〜検証〜レンダリングまでを CLI で完結させる

## 想定ユーザー
- Web 開発者（JSON/TS に慣れている）
- 最小限の依存で試したい人

## スコープ（MVP）
- `laplogos init <dir>`
  - 最小テンプレ生成（schema/設定/サンプル/README）
- `laplogos dev`
  - ローカルレンダリング（watch + 自動リロード）
- `laplogos validate <schema.json>`
  - schema 検証とエラー整形
- `laplogos render <schema.json>`
  - SSR HTML 出力（CI でも利用可）

## 非スコープ（初期）
- エディタ拡張（別フェーズ）
- GUI 操作（CLI の範囲外）
- サーバ公開機能（別ツールに委譲）

## 配布方針（推奨）
- メイン: npm パッケージ（`npx laplogos` で即実行）
- サブ: GitHub Releases（バイナリ配布は将来検討）
- wasm は npm 同梱を基本（オフライン可、再現性重視）

## 実行要件
- Node.js 20.9+（CLI 実行時）
- Rust/wasm-pack は不要（ユーザー側）

## ディレクトリ案
- `project-laplogos-cli/`
  - `src/` CLI 本体
  - `templates/` 初期テンプレ
  - `assets/` wasm/補助ファイル
  - `README.md`

## 技術選定（推奨）
- TypeScript + Node.js
- 引数解析: 軽量ライブラリ（例: commander）
- テスト: `node:test`

## リリース運用
- バージョンは core と同じ（`0.x`）
- タグと changelog を同期
- npm publish は CI で自動化

## セキュリティ方針
- ネットワークは最小（必要時のみ）
- schema 入力は厳格に検証
- ローカルファイル操作は明示的に限定

## フェーズ計画
- Phase 0: validate/render の CLI
- Phase 1: init/dev の追加
- Phase 2: doctor/export などの補助コマンド追加
