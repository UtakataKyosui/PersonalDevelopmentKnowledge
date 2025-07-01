# Dev ContainerでVSCode拡張機能付きのRust開発環境を構築する

> **原文**: [Dev Container で VSCode 拡張機能付きの Rust 開発環境を構築する](https://zenn.dev/codemountains/articles/5995bc3e6b3aa3)

## 概要

ローカル環境にRustをインストールすることなく、Dev ContainerでVSCode拡張機能込みのRust開発環境を構築する方法。MicrosoftのイメージとカスタムDockerfileの両方のアプローチを紹介。

## 前提条件

### 必要なツール
- [Docker](https://www.docker.com/)
- VSCode
- [Dev Containers拡張機能](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)

## アプローチ1: Microsoftイメージを使用

### devcontainer.json（簡単版）
```json
{
  "name": "Rust Sandbox on Dev Container",
  "image": "mcr.microsoft.com/devcontainers/rust:latest",
  "customizations": {
    "vscode": {
      "extensions": [
        "vadimcn.vscode-lldb",
        "fill-labs.dependi",
        "tamasfe.even-better-toml",
        "Swellaby.vscode-rust-test-adapter",
        "JScearcy.rust-doc-viewer",
        "rust-lang.rust-analyzer"
      ]
    }
  }
}
```

### 動作確認
```bash
# RustとCargoがインストールされていることを確認
rustc --version
cargo --help

# Hello worldプロジェクトの作成
cargo init .

# 実行
cargo run
```

### プロジェクト構造
```
.
├── Cargo.lock
├── Cargo.toml
└── src
    └── main.rs
```

## アプローチ2: カスタムDockerfileを使用

### リポジトリ構成
```
.
├── .devcontainer/
│   ├── devcontainer.json
│   └── docker-compose.yml
├── Dockerfile
├── Cargo.toml
└── src/
    └── main.rs
```

### Dockerfile
```dockerfile
FROM rust

# フォーマッターとClippyをインストール
RUN rustup component add rustfmt clippy
```

### docker-compose.yml
```yaml
version: '3'
services:
  sandbox:
    build:
      context: .
      dockerfile: Dockerfile
    working_dir: /workspace
    volumes:
      - .:/workspace
    stdin_open: true
    tty: true
```

### devcontainer.json（Docker Compose版）
```json
{
  "name": "Rust Sandbox on Dev Container",
  "dockerComposeFile": "../docker-compose.yml",
  "service": "sandbox",
  "workspaceFolder": "/workspace",
  "customizations": {
    "vscode": {
      "extensions": [
        "ms-azuretools.vscode-docker",
        "vadimcn.vscode-lldb",
        "fill-labs.dependi",
        "tamasfe.even-better-toml",
        "Swellaby.vscode-rust-test-adapter",
        "JScearcy.rust-doc-viewer",
        "rust-lang.rust-analyzer"
      ]
    }
  }
}
```

## Cargo Workspaceの活用

### Workspace構成
```toml
# Cargo.toml
[workspace]
members = ["app", "lib"]

[[workspace.metadata.bin]]
name = "app"
path = "app"

[workspace.dependencies]
# 共通の依存関係を定義
```

### メリット
- **複数アプリの管理**: 複数のアプリケーションを追加可能
- **ビルド時間短縮**: ライブラリクレートに分割してビルド最適化
- **汎用性**: 柔軟なプロジェクト構成
- **デフォルト実行**: `app`をデフォルトターゲットに設定

## 推奨VSCode拡張機能

### 1. rust-analyzer（必須）
```json
"rust-lang.rust-analyzer"
```
- Rust公式のLanguage Server Protocol (LSP)
- コード補完、シンタックスハイライト
- どの言語でもLSPは必須レベル

### 2. CodeLLDB（デバッグ）
```json
"vadimcn.vscode-lldb"
```
- コンパイル言語デバッグ用拡張機能
- ブレークポイントでのデバッグ
- 開発効率向上に必須

### 3. Rust Test Explorer（テスト）
```json
"Swellaby.vscode-rust-test-adapter"
```
- VS Codeサイドバーからテスト表示・実行
- テストコードの一覧表示
- 個別テスト実行が可能

### 4. Rust Doc Viewer（ドキュメント）
```json
"JScearcy.rust-doc-viewer"
```
- ローカル生成プロジェクトドキュメントの表示
- 新しいウィンドウで簡単参照
- API仕様の確認が容易

### 5. Dependi（依存関係管理）
```json
"fill-labs.dependi"
```
- クレート（ライブラリ）管理の効率化
- `crates`拡張機能の後継（メンテナンス終了のため）
- 依存関係の更新・管理が楽

### 6. Even Better TOML（設定ファイル）
```json
"tamasfe.even-better-toml"
```
- TOMLファイルでのコード補完
- Cargo.toml編集時に便利
- 設定ファイルの記述ミス防止

## 開発ワークフロー

### 基本的な開発サイクル
```bash
# プロジェクト初期化
cargo init .

# 依存関係の追加
cargo add serde

# コードの実行
cargo run

# テストの実行
cargo test

# フォーマット
cargo fmt

# リンター実行
cargo clippy
```

### VSCode内での操作
- **RUNボタン**: `fn main()`の上に表示されるRUNボタンから実行
- **デバッグ**: ブレークポイント設定でステップ実行
- **テスト実行**: サイドバーのTest Explorerから個別実行
- **ドキュメント**: Rust Doc Viewerで関数仕様確認

## 環境の利点

### 開発効率
- **すぐに試せる**: ローカルインストール不要
- **不要なら破棄可能**: クリーンな環境維持
- **再現性**: チーム全体で同じ環境

### 学習コスト削減
- **Rust環境構築不要**: Dockerがあれば即開始
- **IDE設定済み**: 拡張機能が自動インストール
- **ベストプラクティス**: 推奨設定がプリセット

### メンテナンス性
- **バージョン管理**: Dockerfileでバージョン固定
- **チーム共有**: `.devcontainer`でチーム環境統一
- **アップデート容易**: イメージ更新で最新環境

## GitHubサンプルリポジトリ

### 活用可能なサンプル
- [codemountains/rusty-sandbox](https://github.com/codemountains/rusty-sandbox)

### クローンして即利用
```bash
git clone https://github.com/codemountains/rusty-sandbox.git
cd rusty-sandbox
# VS CodeでDev Container起動
```

## まとめ

Dev ContainerによるRust開発環境は以下を実現：

- **ゼロセットアップ**: ローカル環境を汚さない
- **すぐに破棄可能**: 実験的なコードも安心
- **チーム統一**: 同じ開発環境をチーム共有
- **最適化済み**: Rust開発に最適な拡張機能プリセット
- **公式サポート**: rust-analyzerなど公式ツール利用

特に「すぐに試せる、不要なら破棄できる」のがDev Containerの最大の魅力。Rust学習やプロトタイプ開発に最適。