# Project Aix (MVP)

**"Human-Unreadable, Machine-Optimal"**
AI ネイティブ時代の次世代 UI ランタイム＆開発言語プロジェクト。

## コンセプト

- **AI First:** 人間の可読性より、AI の生成効率（トークン削減）を最優先。
- **No Build Time:** AI が生成したデータを Rust エンジンが即座に実行。
- **Safety:** 任意のコード実行を禁止し、定義された Action のみを許可。

## 技術スタック (Rust-Centric)

- **Core Engine:** Rust (WebAssembly)
- **Host App:** Next.js (React)
- **Schema:** Aix Schema v1 (JSON based)

## ディレクトリ構成

- `aix_core/`: Rust 製ランタイムエンジン (Wasm)
- `aix_web/`: フロントエンド (Next.js) + デバッグ可視化ツール

## 開発環境セットアップ

### 前提条件

- WSL2 (Ubuntu)
- Rust (`cargo`, `wasm-pack`)
- Node.js (v18+)

### 起動手順

1. **Rust エンジンのビルド**
   ```bash
   cd aix_core
   wasm-pack build --target web
   ```
