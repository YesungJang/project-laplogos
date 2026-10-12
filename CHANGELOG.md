# Changelog

## Unreleased

- Core: `LaplogosEngine.get_state` を追加（state データの決定的・読み取り専用スナップショット）
- Core: `LaplogosEngine.find` を追加（materialized ツリーを ID 完全一致で検索する決定的・読み取り専用スナップショット）
- Core: `LaplogosEngine.inspect` を追加（materialize 済み構造の決定的・読み取り専用スナップショット）
- Web: Error View を `role="alert"` で通知し、メッセージはプレーンテキストのまま表示するように変更
- Web: Error View に復旧手順（再読み込み／Debug View 確認）を 1 文追加し、SSR と CSR の構造を一致
- Web: `renderErrorHtml` の Node テストに正常系・境界（空／長文／多バイト）・不正入力（HTML 混入）を追加
- Docs: Core 公開 API のドキュメント `docs/core-api.md` を追加
- Docs: `docs/core-api.md` に `inspect` と各取得 API の役割の違いを追記
- Docs: `docs/preview/observability.md` に Error View の表示仕様（role=alert / プレーンテキスト / 復旧手順）を追記

## v0.1.0 (beta) - 2025-12-31

- Core: materialize/diff/patch 基盤、ID 制約、ガードレール（max nodes/loop/depth）
- Web: SSR/CSR/Hydration の整備、Error/Debug View、再現パッケージ
- Security: fetch 制約、allowed_hosts/actions、Lnk の危険スキーム拒否
- Tooling: CI とテスト整備、バージョン整合チェック
