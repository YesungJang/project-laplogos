# Core Public API

`laplogos_core` が公開する API をまとめます。
Wasm バインディングは Rust 側の名前をそのまま公開します（snake_case）。
JS からも `engine.get_state()` のように同じ名前で呼び出します。

## LaplogosEngine

| メソッド | 戻り値 | 内容 |
| --- | --- | --- |
| `new(initial_json)` | `LaplogosEngine` | schema v を検証し Normalize したエンジンを生成 |
| `init(config_json)` | `()` | `allowed_actions` / `allowed_hosts` を設定 |
| `set_on_nav(callback)` | `()` | `nav` Action のコールバックを登録 |
| `dispatch(action_json)` | `()` | Action を実行 |
| `set_page_json(page_json)` | `()` | schema を差し替え |
| `set_state_json(state_json)` | `()` | state を差し替え |
| `rerender(container_id)` | `()` | Materialize → Diff → Patch |
| `hydrate(container_id)` | `()` | SSR 済み DOM にイベントのみ接続 |
| `get_state()` | `String` | **state データの決定的な読み取り専用スナップショット** |
| `get_state_json()` | `String` | state データをそのままシリアライズ（既存） |
| `get_page_json()` | `String` | 正規化後の schema |
| `get_input_page_json()` | `String` | 正規化前の入力 schema |
| `get_action_log_json()` | `String` | Action ログ（JSON 配列） |
| `get_error_log_json()` | `String` | エラー/警告ログ（JSON 配列） |
| `get_metrics_json()` | `String` | 計測値（normalize / materialize / diff / patch / hydrate） |

### get_state

```ts
const json = engine.get_state();
// => '{"count":1,"profile":{"first":"A","last":"B"}}'
```

- 戻り値は **state データのみ**を含む JSON オブジェクトの文字列です。
  state が空なら `"{}"` を返します。
- **決定的**: オブジェクトのキーは再帰的に昇順（Rust の文字列順＝UTF-8 バイト順）へ
  ソートされます。`state` の内部表現は `HashMap` なので、同じ内容でも挿入順によって
  列挙順が変わりますが、`get_state()` の出力は同じ state データに対して常に同一です。
  配列は要素順を保持します（ソートしません）。
- **読み取り専用**: state もログも metrics も含め、エンジンの観測可能な状態を一切変更
  しません。返るのはディープコピーなので、呼び出し側が値を加工してもエンジンには影響
  しません。
- **state データ以外は返しません**: materialize/render 済みノード、入力 schema、正規化後の
  schema は含まれません。それらが必要な場合は `get_page_json()` /
  `get_input_page_json()` を使用してください。

### get_state との違い（get_state_json）

`get_state_json()` は従来どおり `state` を直接シリアライズします。含まれるデータは
`get_state()` と同じですが、キー順は保証されません。既存の呼び出しは変更不要です。

## モジュール関数

- `init_panic_hook()`: panic を console へ出力
- `render_laplogos(json_str, container_id)`: state なしで 1 回だけ描画
- `render_laplogos_to_string(page, state)`: SSR 用の HTML 文字列を生成

## Rust から使う場合

クレートとしては `LaplogosPage` / `LaplogosNode` / `LaplogosState` と
`normalize_page` / `render_to_string`、`materialize` / `diff` / `normalize` モジュールを
公開しています。state スナップショットは `LaplogosEngine` を介さずに利用できます。

```rust
use laplogos_core::LaplogosState;

let state = LaplogosState::default();
let value = state.snapshot_value(); // serde_json::Value（detach 済み）
let json = state.snapshot_json();   // String（get_state と同じ出力）
```

`LaplogosEngine::get_state()` は `LaplogosState::snapshot_json()` の薄いラッパーです。
