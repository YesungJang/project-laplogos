# Changelog

## Unreleased

- Core: `LaplogosEngine.get_state` を追加（state データの決定的・読み取り専用スナップショット）
- Core: `LaplogosEngine.find` を追加（materialized ツリーを ID 完全一致で検索する決定的・読み取り専用スナップショット）
- Core: `LaplogosEngine.inspect` を追加（materialize 済み構造の決定的・読み取り専用スナップショット）
- Web: Error View（CSR / SSR）にアラートロールと復旧手順を追加（メッセージは常にプレーンテキスト、文字列でない値や空白は代替文へフォールバック）
- Docs: Core 公開 API のドキュメント `docs/core-api.md` を追加
- Docs: `docs/core-api.md` に `inspect` と各取得 API の役割の違いを追記
- Docs: `docs/ssr-csr-hydration.md` に Error 表示（SSR / CSR 共通）の振る舞いを追記

### 作業ログ（Error 表示の改善）

- 変更: `ErrorView` と SSR の `renderErrorHtml` に `role="alert"`（`aria-live` / `aria-atomic` 付き）と復旧手順を追加し、メッセージをプレーンテキスト固定―不正な値は代替文へフォールバック。共有定数と正規化は `lib/laplogos-error-view.ts` に集約し、`message` props と既存の呼び出しはそのまま維持。
- 検証: 変更後に `npm --prefix laplogos_web run test:node` / `run lint` / `run build` を実行し、いずれも PASS。正常系・境界値（空文字列 / 長文 / HTML や引用符を含む文字列 / 非 ASCII）・不正入力を CSR と SSR の両方でテスト。
- 未実施: `docs/preview/observability.md` の Error View policy は変更許可パス外のため未更新。本作業ログも `codex-history.md` が許可パス外のため、本ファイルに記載している。

## v0.1.0 (beta) - 2025-12-31

- Core: materialize/diff/patch 基盤、ID 制約、ガードレール（max nodes/loop/depth）
- Web: SSR/CSR/Hydration の整備、Error/Debug View、再現パッケージ
- Security: fetch 制約、allowed_hosts/actions、Lnk の危険スキーム拒否
- Tooling: CI とテスト整備、バージョン整合チェック
