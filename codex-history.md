2025-12-18 23:39 タスク 1.1 依存関係を Cargo.toml に追加
2025-12-18 23:40 タスク 1.2 スキーマ構造体を lib.rs に実装
2025-12-18 23:41 タスク 1.3 renderer.rs で DOM レンダラーを実装
2025-12-18 23:43 タスク 1.4 エントリーポイントを lib.rs に追加
2025-12-18 23:46 cargo check エラーを修正(web-sys に version 追加・AixPage に derive 追加)
2025-12-18 23:51 タスク 2.1 wasm-pack build を実行
2025-12-18 23:52 タスク 2.2 Next.js の Webpack で asyncWebAssembly を有効化
2025-12-18 23:52 タスク 2.3 useAixEngine フックを作成
2025-12-18 23:53 タスク 2.4 page.tsx でテスト JSON を render_aix する実装に変更
2025-12-18 23:57 タスク 2.5 Turbopack 利用時の webpack 警告回避で turbopack 設定を追加
2025-12-19 00:00 useAixEngine の pkg import パスを修正
2025-12-19 00:03 Turbopack で外部ディレクトリを許可し pkg パスを再修正
2025-12-19 00:07 Turbopack 向けに aix_core を file 依存として追加し import をパッケージ参照に変更
2025-12-19 00:10 Turbopack 用に aix_core への resolveAlias と transpile 設定を追加
2025-12-19 00:13 next.config.ts を簡素化し ESM エラーの原因になりそうな alias 処理を除去
2025-12-19 00:15 useAixEngine のインポートを相対パスに戻し turbopack 設定最小化で対応
2025-12-19 00:24 webpack フラグで dev を固定し aix_core をパッケージ名インポートに戻して再構成
2025-12-19 00:29 wasm エクスポートに#[wasm_bindgen]を付与し pkg を再ビルド
2025-12-19 00:32 useAixEngine で Wasm 初期化後に init_panic_hook を呼ぶよう修正
2025-12-20 11:55 タスク 3.1 store.rs に AixState/Action/reduce を実装
2025-12-20 11:56 タスク 3.2 AixEngine 構造体を lib.rs に追加し状態保持を実装
2025-12-20 12:00 タスク 3.3 dispatch で Action JSON を reduce に渡す処理を追加
2025-12-20 12:01 タスク 3.4 renderer に update_dom と rerender 連携を追加
2025-12-20 12:03 タスク 4.1 DebugView コンポーネントを追加
2025-12-20 12:04 タスク 4.2 useAixEngine を AixEngine 生成・dispatch ラッパー付きに拡張
2025-12-20 12:05 タスク 4.3 page.tsx を 2 カラム構成にし DebugView と dispatch ボタンを統合
2025-12-20 12:15 layout.tsx で suppressHydrationWarning を追加し拡張由来の属性差分を無視
2025-12-20 12:23 wasm-pack を再ビルドして AixEngine を pkg に含めた
2025-12-20 20:45 タスク 5.0 ActionSpec 正規化とバリデーション/State 型変更を実装
2025-12-20 20:47 タスク 5.1 resolve_text と MaterializeScope を materialize.rs に追加
2025-12-20 20:49 タスク 5.2 resolve_if を追加し非 bool 判定を warn 扱いに
2025-12-20 20:52 タスク 5.3 ID 制約チェックと conformance テストを追加
2025-12-20 20:57 タスク 5.4 Inp 対応とイベント dispatch 経路を追加
2025-12-20 21:02 タスク 5.5 materialize_many/one_root とループ/if/置換処理を実装
2025-12-20 21:05 タスク 6.0 materialize を使った更新ループと last_materialized 保存を追加
2025-12-20 21:07 タスク 6.1 diff.rs で Patch 定義と差分計算を実装
2025-12-20 21:10 タスク 6.2 data-aix-id 付与と Patch 適用/フォールバックを実装
2025-12-20 21:11 タスク 6.3 UpdateAttr の value 更新時にフォーカスガードを追加
2025-12-20 21:14 タスク 7.1 Capability 設定と allowed_actions/hosts チェックを追加
2025-12-20 21:17 タスク 7.2 fetch アクションの非同期実行と allowed_hosts 検証を実装
2025-12-20 21:20 タスク 8.1 conformance テストと tests/data の valid/invalid JSON を追加
2025-12-20 21:22 タスク 8.2 render_to_string と snapshot テストを追加
2025-12-20 21:31 spec 実行で公開 API を調整しテスト通過
2025-12-20 21:37 Task5 以降の Spec チェックと cargo test 実行
2025-12-20 23:06 R1 Laplogos 残骸の棚卸し結果を整理
2025-12-20 23:10 R2 ディレクトリを laplogos_core/laplogos_web へ改名
2025-12-20 23:12 R3 crate 名を laplogos_core へ変更し wasm-pack ビルド
2025-12-20 23:14 R4 render_laplogos を render_laplogos に改名
2025-12-20 23:18 R5 Rust の Laplogos*型を Laplogos*に改名し cargo check
2025-12-20 23:22 R6 Web フックと表示文言を laplogos へ改名
2025-12-20 23:26 R7 wasm 参照と web 設定/依存を laplogos_core へ追従
2025-12-20 23:28 R8 テスト/フィクスチャの Laplogos 参照を確認し laplogos に統一
2025-12-20 23:30 R9 README/DEV_PLAN の Laplogos 表記を laplogos へ統一
2025-12-20 23:41 R10 旧名残骸の最終クリーンアップ
2025-12-20 23:48 R11 フルビルド検証とテスト実行
2025-12-20 23:54 Task5-8 Spec 再実行（cargo test）
2025-12-21 00:00 Dispatch random count の ActionSpec を修正
2025-12-21 00:32 MCP 設定ファイルを確認
2025-12-21 00:37 MCP 設定の再確認
2025-12-21 00:39 MCP 接続再確認
2025-12-21 00:46 MCP 接続の確認を実施
2025-12-21 01:19 MCP 接続状況を確認
2025-12-21 01:52 PR 作成・マージ完了
2025-12-21 02:46 Task 9.1: Packed Schema 定義を laplogos_core に追加
2025-12-21 02:48 Task 9.2: packed->v1 の normalize 実装を追加
2025-12-21 02:50 Task 9.3: packed 用 conformance テストを追加
2025-12-21 02:51 Task 9.4: Engine 初期化/更新で normalize を通すよう変更
2025-12-21 02:52 Task 11.1: SSR 用 render_laplogos_to_string を追加
2025-12-21 02:53 Task 11.2: SSR で HTML 生成と安全な bootstrap JSON 埋め込み
2025-12-21 02:56 Task 11.3: Hydration 用のイベント bind と起動フローを追加
2025-12-21 02:56 Task 11.4: 最小限の suppressHydrationWarning を追加
2025-12-21 09:08 Task 10.1: nav Action と on_nav コールバックを追加
2025-12-21 09:09 Task 10.2: Lnk ノードの描画と nav イベントを追加
2025-12-21 09:10 Task 10.3: on_nav 連携とルート変化でのページ更新を追加
2025-12-21 09:10 Task 10.4: popstate 対応として path 変化で再読み込み
2025-12-21 09:16 SSR 時の Wasm 初期化を Node 対応にして 500 回避
2025-12-21 09:36 dev の 500 回避のため Wasm 初期化と webpack 設定を調整
2025-12-21 09:46 dev 警告と/api/laplogos 404 解消のため SSR 初期化と API を調整
2025-12-21 09:48 エラー対応による設計意図の逸脱有無を確認
2025-12-22 03:10 設計意図に対する実装状況の確認と回答
2025-12-28 23:43 旧 render_laplogos 経路を materialize 必須に変更
2025-12-28 23:44 SSR 埋め込み JSON の<エスケープを明示
2025-12-28 23:44 ?page=の URL パス統一のため正規化ヘルパーを共有
2025-12-30 01:27 Phase 13: Schema互換性ポリシーを文書化
2025-12-30 01:27 Phase 13: SemVer運用方針とバージョン整合性チェックを追加
2025-12-30 01:27 Phase 13: サポート対象環境を文書化
2025-12-30 01:27 Phase 13: 配布形態とwasm配布経路を固定
2025-12-30 01:27 Phase 14: 脅威モデルを文書化
2025-12-30 01:27 Phase 14: 入力バリデーションの仕様化とガードレールを実装
2025-12-30 01:27 Phase 14: Capabilityデフォルト方針を反映
2025-12-30 01:27 Phase 14: SSRセキュリティ仕様を文書化
2025-12-30 01:27 Phase 14: 依存関係チェック方針を文書化
2025-12-30 01:27 Phase 15: ログ設計とDebug View拡張を実装
2025-12-30 01:27 Phase 15: Error View最小UIを追加
2025-12-30 01:27 Phase 15: 再現パッケージのCopy/Downloadを追加
2025-12-30 01:27 Phase 15: SSR/CSR表示情報を追加
2025-12-30 01:27 Phase 16: ベンチ/負荷シナリオを文書化
2025-12-30 01:27 Phase 16: 計測ポイントとPerf表示を追加
2025-12-30 01:27 Phase 16: 主要パフォーマンス改善を適用
2025-12-30 01:27 Phase 16: Preview制約を明文化
2025-12-30 01:27 Phase 17: ビルド成果物と手順を固定
2025-12-30 01:27 Phase 17: 環境前提を文書化
2025-12-30 01:27 Phase 17: CI設定とバージョンチェックを追加
2025-12-30 01:27 Phase 17: Previewデプロイ手順を固定
2025-12-30 01:27 Phase 18: READMEのGetting Startedを更新
2025-12-30 01:27 Phase 18: Schema v1仕様書を追加
2025-12-30 01:27 Phase 18: セキュリティ/Capability説明を追加
2025-12-30 01:27 Phase 18: SSR/CSR/Hydration説明を追加
2025-12-30 01:27 Phase 18: サンプルJSONを追加
2025-12-30 01:27 Phase 18: Preview制約/既知制限を記載
2025-12-30 11:05 ベータ公開前の全体評価と懸念点をレビュー
2025-12-30 11:13 ベータ公開向けにSSR/設定/バージョン周りの不具合を修正
2025-12-30 11:42 ベータ公開前の全体評価と懸念点を再レビュー
2025-12-30 12:11 ベータ公開に向けた改善提案を整理
2025-12-30 12:22 改善提案の実装（ID一意性、コンテナID、SSRキャッシュ、設定ブートストラップ等）
2025-12-30 12:26 ベータ公開向けの全体評価と懸念点レビュー
2025-12-30 12:39 ベータ公開に対する所感と懸念の共有
2025-12-30 13:00 ベータ懸念点の対策（ノーパッチ抑制、fetch事前サイズチェック、デバッグ可視化）
