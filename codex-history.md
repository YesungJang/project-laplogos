2025-12-18 23:39 タスク1.1 依存関係をCargo.tomlに追加
2025-12-18 23:40 タスク1.2 スキーマ構造体をlib.rsに実装
2025-12-18 23:41 タスク1.3 renderer.rsでDOMレンダラーを実装
2025-12-18 23:43 タスク1.4 エントリーポイントをlib.rsに追加
2025-12-18 23:46 cargo checkエラーを修正(web-sysにversion追加・AixPageにderive追加)
2025-12-18 23:51 タスク2.1 wasm-pack buildを実行
2025-12-18 23:52 タスク2.2 Next.jsのWebpackでasyncWebAssemblyを有効化
2025-12-18 23:52 タスク2.3 useAixEngineフックを作成
2025-12-18 23:53 タスク2.4 page.tsxでテストJSONをrender_aixする実装に変更
2025-12-18 23:57 タスク2.5 Turbopack利用時のwebpack警告回避でturbopack設定を追加
2025-12-19 00:00 useAixEngineのpkg importパスを修正
2025-12-19 00:03 Turbopackで外部ディレクトリを許可しpkgパスを再修正
2025-12-19 00:07 Turbopack向けにaix_coreをfile依存として追加しimportをパッケージ参照に変更
2025-12-19 00:10 Turbopack用にaix_coreへのresolveAliasとtranspile設定を追加
2025-12-19 00:13 next.config.tsを簡素化しESMエラーの原因になりそうなalias処理を除去
2025-12-19 00:15 useAixEngineのインポートを相対パスに戻しturbopack設定最小化で対応
2025-12-19 00:24 webpackフラグでdevを固定しaix_coreをパッケージ名インポートに戻して再構成
2025-12-19 00:29 wasmエクスポートに#[wasm_bindgen]を付与しpkgを再ビルド
2025-12-19 00:32 useAixEngineでWasm初期化後にinit_panic_hookを呼ぶよう修正
2025-12-20 11:55 タスク3.1 store.rsにAixState/Action/reduceを実装
2025-12-20 11:56 タスク3.2 AixEngine構造体をlib.rsに追加し状態保持を実装
2025-12-20 12:00 タスク3.3 dispatchでAction JSONをreduceに渡す処理を追加
2025-12-20 12:01 タスク3.4 rendererにupdate_domとrerender連携を追加
2025-12-20 12:03 タスク4.1 DebugViewコンポーネントを追加
2025-12-20 12:04 タスク4.2 useAixEngineをAixEngine生成・dispatchラッパー付きに拡張
2025-12-20 12:05 タスク4.3 page.tsxを2カラム構成にしDebugViewとdispatchボタンを統合
2025-12-20 12:15 layout.tsxでsuppressHydrationWarningを追加し拡張由来の属性差分を無視
2025-12-20 12:23 wasm-packを再ビルドしてAixEngineをpkgに含めた
2025-12-20 20:45 タスク5.0 ActionSpec正規化とバリデーション/State型変更を実装
2025-12-20 20:47 タスク5.1 resolve_textとMaterializeScopeをmaterialize.rsに追加
2025-12-20 20:49 タスク5.2 resolve_ifを追加し非bool判定をwarn扱いに
2025-12-20 20:52 タスク5.3 ID制約チェックとconformanceテストを追加
2025-12-20 20:57 タスク5.4 Inp対応とイベントdispatch経路を追加
2025-12-20 21:02 タスク5.5 materialize_many/one_rootとループ/if/置換処理を実装
2025-12-20 21:05 タスク6.0 materializeを使った更新ループとlast_materialized保存を追加
2025-12-20 21:07 タスク6.1 diff.rsでPatch定義と差分計算を実装
