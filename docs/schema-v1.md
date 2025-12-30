# Schema v1

## Top-level

- `v`: schema version（未指定は v1）
- `root`: ルートノード（常に 1 ノード）
- `defs` / `styles`: packed schema 用（任意）

## Node fields

- `i`: ID（`[A-Za-z0-9_:-]+`）
- `t`: type（`Box` / `Txt` / `Btn` / `Inp` / `Lnk`）
- `p`: props（任意）
- `c`: children（任意）
- `if`: boolean 判定キー（`$key` 形式）
- `for`: 配列展開（文字列 or 数値配列のみ）
- `on`: イベント → Action 配列

## Props（主要）

- `Txt.p.val`: 表示文字列
- `Btn.p.label`: ボタン表示
- `Inp.p.val`: **`$key` 必須**（入力は `set` を発行）
- `Lnk.p.href` / `Lnk.p.label`

## ActionSpec

- `set`: `key` 必須、`val` または `val_from`（`event.value` のみ）
- `inc`: `key` 必須、`by` 任意
- `fetch`: `url` 必須、`on_success` / `on_fail` は Action 配列
- `nav`: `to` 必須（Core は遷移せず `on_nav` を呼ぶ）

## 置換ルール

- `"$key"` の完全一致のみを置換対象にする（部分一致は無視）
- ループ内は `"$item"` / `"$key"` が有効

## Packed schema（Def/Ref + Style）

- `defs`: ノード定義（`i` は持たない前提）
- `r`: defs 参照
- `st`: style 参照（`styles[st]` を props に不足分だけマージ）
- `o`: props 上書き専用（`p` 以外の上書きは禁止）
