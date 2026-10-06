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
| `inspect()` | `String` | **現在の materialize 済み構造の決定的な読み取り専用スナップショット** |
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
  schema は含まれません。それらが必要な場合は `inspect()` / `get_page_json()` /
  `get_input_page_json()` を使用してください。

### get_state との違い（get_state_json）

`get_state_json()` は従来どおり `state` を直接シリアライズします。含まれるデータは
`get_state()` と同じですが、キー順は保証されません。既存の呼び出しは変更不要です。

### inspect

```ts
const json = engine.inspect();
// => '{"c":[{"c":null,"for":null,"i":"t1","if":null,"o":null,"on":null,"p":{"val":"Hello"},"r":null,"st":null,"t":"Txt"}],...}'
```

- 戻り値は **現在の materialize 済み構造**（root とその children）を含む JSON オブジェクトの
  文字列です。`p` は置換適用後の値（`$key` / `$item` / `$key` の解決結果、`Inp` のバインド
  解決結果）、`i` は materialize 後の ID（ループ展開後の `row:a` / `row:a:label` など）です。
  表現は既存の `LaplogosNode` のシリアライズ形式をそのまま使います（新しい IR や
  メタデータ用のラッパーは導入しません）。
- **現在の正規化後 schema と state から materialize します**。最後に描画したツリーや DOM は
  参照しないため、`rerender()` / `hydrate()` を一度も呼んでいなくても利用できます。
- **決定的**: オブジェクトのキーは再帰的に昇順ソートされます。配列と children は
  materialize 時の順序を保持します（ソートしません）。同じ schema と state に対して出力は
  常に同一です。
- **読み取り専用**: 成功時も失敗時も、state / 正規化後 schema / 入力 schema / 直前の
  materialize 結果 / Action ログ / エラーログ / metrics / コールバック / dispatch の記録を
  一切変更しません。DOM 操作・描画・イベント登録・通信・console 出力も行いません。
  返るのはディープコピーなので、呼び出し側が値を加工してもエンジンには影響しません。
- **materialize のエラーはログではなくエラーとして返します**: ガードレール超過
  （max nodes / max depth / loop expand）、ID 重複や不正な ID、root の `if` が false の場合は
  エラーになります（JS では例外、Rust では `Err`）。`rerender()` などと違い、エラーログには
  記録されません。
- **`if` が boolean でない場合**: 対象が**子ノード**なら従来どおり非表示になるだけで、
  エラーにはなりません（この API は警告を公開しません）。一方、**root の `if`** が boolean で
  ない場合は非表示扱いになった結果 `root if=false is fatal` としてエラーになります。

### 役割の違い（get_state / get_page_json / inspect）

| メソッド | 返すもの |
| --- | --- |
| `get_state()` | state データのみ（schema もノードも含まない） |
| `get_input_page_json()` | 正規化前の入力 schema |
| `get_page_json()` | 正規化後の schema（if/for は未展開、`$key` は未置換） |
| `inspect()` | 現在の materialize 済み構造（if/for 展開後、`$key` 置換後） |

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

materialize 済み構造のスナップショットも同様に、`LaplogosEngine` を介さずに利用できます。

```rust
use laplogos_core::{inspect_snapshot_json, inspect_snapshot_value, normalize_page, LaplogosState};

let page = normalize_page(raw).expect("normalize page");
let state = LaplogosState::default();

let value = inspect_snapshot_value(&page.root, &state); // Result<serde_json::Value, LaplogosError>
let json = inspect_snapshot_json(&page.root, &state);   // Result<String, LaplogosError>（inspect と同じ出力）
```

`LaplogosEngine::inspect()` は `inspect_snapshot_json()` の薄いラッパーです。
