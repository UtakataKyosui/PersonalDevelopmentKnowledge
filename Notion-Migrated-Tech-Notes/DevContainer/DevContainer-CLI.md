# DevContainer CLI

## メタデータ
- **作成日時**: 2024-07-26T04:54:00.000Z
- **最終更新**: 2024-07-26T07:02:00.000Z
- **タグ**: #DevContainer #CLI #Development
- **移植元**: Notion
- **移植日**: 2025-07-02

## 概要

DevContainerをコマンドラインから操作するためのCLIツールについて記載したページです。

## DevContainer CLIとは

Dev Container CLIは、Development Containers機能をコマンドラインから利用するためのツールです。
Visual Studio Codeを使わずに、ターミナルから直接DevContainerを操作できます。

## インストール方法

```bash
npm install -g @devcontainers/cli
```

## 主要コマンド

### up コマンド
DevContainerを起動します。
```bash
devcontainer up --workspace-folder .
```

### exec コマンド
DevContainer内でコマンドを実行します。
```bash
devcontainer exec --workspace-folder . -- command
```

### build コマンド
DevContainerイメージをビルドします。
```bash
devcontainer build --workspace-folder .
```

### features コマンド
利用可能なfeaturesを表示します。
```bash
devcontainer features list
```

## 使用例

### 基本的な使用方法
```bash
# DevContainerを起動
devcontainer up --workspace-folder /path/to/project

# コンテナ内でコマンド実行
devcontainer exec --workspace-folder /path/to/project -- npm install

# コンテナに接続
devcontainer exec --workspace-folder /path/to/project -- bash
```

### CI/CDでの活用
```yaml
# GitHub Actions例
- name: Setup DevContainer
  run: |
    npm install -g @devcontainers/cli
    devcontainer up --workspace-folder .
    
- name: Run tests
  run: |
    devcontainer exec --workspace-folder . -- npm test
```

## 設定ファイル

DevContainer CLIは `.devcontainer/devcontainer.json` の設定を使用します。

```json
{
  "name": "My Dev Container",
  "image": "mcr.microsoft.com/devcontainers/base:ubuntu",
  "features": {
    "ghcr.io/devcontainers/features/node:1": {
      "version": "18"
    }
  }
}
```

## 応用例

### 自動化スクリプト
```bash
#!/bin/bash
# DevContainer自動セットアップスクリプト

echo "Setting up DevContainer..."
devcontainer up --workspace-folder .

echo "Installing dependencies..."
devcontainer exec --workspace-folder . -- npm install

echo "Running initial setup..."
devcontainer exec --workspace-folder . -- npm run setup

echo "Setup complete!"
```

## 関連リンク
- [[DevContainer-Proxy]] - Proxy環境下での注意点
- [[Bun-DevContainer]] - Bun DevContainer設定
- [[Prisma-DevContainer]] - Prisma DevContainer設定
- [[Rust-DevContainer-WebAssembly開発]] - Rust開発環境

## ノート
- このページはNotionから移植されました
- DevContainer CLIは継続的にアップデートされているため、最新情報を確認してください
- CI/CDパイプラインでの活用により、開発環境の一貫性を保つことができます
