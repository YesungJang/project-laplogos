# Changelog

## Unreleased

- Core: `LaplogosEngine.get_state` を追加（state データの決定的・読み取り専用スナップショット）
- Core: `LaplogosEngine.find` を追加（materialized ツリーを ID 完全一致で検索する決定的・読み取り専用スナップショット）
- Docs: Core 公開 API のドキュメント `docs/core-api.md` を追加

## v0.1.0 (beta) - 2025-12-31

- Core: materialize/diff/patch 基盤、ID 制約、ガードレール（max nodes/loop/depth）
- Web: SSR/CSR/Hydration の整備、Error/Debug View、再現パッケージ
- Security: fetch 制約、allowed_hosts/actions、Lnk の危険スキーム拒否
- Tooling: CI とテスト整備、バージョン整合チェック
