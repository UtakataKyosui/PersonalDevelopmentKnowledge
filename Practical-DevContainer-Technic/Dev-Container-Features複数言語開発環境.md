# Dev Container Featuresで複数言語開発環境を構築

> **原文**: [1リポジトリで2つ以上の開発言語のローカル開発環境をDevcontainerで作るとき、Dev Container Featuresが便利](https://zenn.dev/nmemoto/articles/dev-container-features-are-useful)

## 概要

1つのリポジトリで複数の開発言語（Go+Python）を管理する際のDev Container環境構築について、Dev Container Features（プレビュー機能）を活用したアプローチの検証と実装。3つの構成案を比較検討し、最も実用的な方法を特定。

## 検討した3つの構成案

### 1. Connect to multiple containers
- **概要**: 開発言語毎に任意設定のコンテナを作成
- **問題点**: 
  - 各コンテナごとにVSCodeウィンドウが必要
  - プロジェクトルート直下ファイルの編集が困難
  - ウィンドウ間の行き来が面倒
  - Multi-root Workspacesでも設定が複雑

### 2. カスタムDockerfile
- **概要**: devcontainer.json指定のDockerfile内に複数言語設定を記述
- **特徴**: 高いカスタマイズ性、完全制御可能

### 3. Dev Container Features（採用）
- **概要**: 予め定義された設定を導入し、一つのコンテナを作成
- **メリット**: シンプルな設定、即座に開発可能

## Dev Container Featuresとは

### 基本概念
- **仕様**: [Development Container Specification](https://containers.dev/implementors/features/)で規定
- **定義**: インストールコードと開発コンテナ設定の自己充足的で共有可能な単位
- **状態**: プレビュー機能（2023年時点）

### 主要メリット
- **設定簡素化**: Dockerfileにapt-get installやcurl等を書く必要なし
- **即時開発**: VSCode設定済みで使用開始可能
- **組み合わせ自由**: 複数言語・ツールの同時導入

## 実装例：Go + Python環境

### .devcontainer.json
```json
{
  "name": "go-and-python",
  "image": "mcr.microsoft.com/devcontainers/base:debian",
  "shutdownAction": "stopContainer",
  "remoteUser": "vscode",
  "features": {
    "ghcr.io/devcontainers/features/python:1": {
      "version": "latest",
      "installJupyterlab": true
    },
    "ghcr.io/devcontainers/features/go:1": {
      "version": "latest"
    }
  },
  "postStartCommand": "pip install -r python-workspace/requirements.txt"
}
```

### 利用可能なFeatures

#### 主要プログラミング言語
- **Python**: python
- **Go**: go  
- **Rust**: rust
- **Java**: java
- **Ruby**: ruby
- **Node.js**: node

#### 開発ツール
- **GitHub CLI**: github-cli
- **Docker in Docker**: docker-in-docker
- **Git**: git

### 複数言語設定例

#### フルスタック開発環境
```json
{
  "features": {
    "ghcr.io/devcontainers/features/python:1": {
      "version": "latest"
    },
    "ghcr.io/devcontainers/features/go:1": {
      "version": "latest"
    },
    "ghcr.io/devcontainers/features/ruby:1": {
      "version": "latest"
    },
    "ghcr.io/devcontainers/features/java:1": {
      "version": "latest"
    },
    "ghcr.io/devcontainers/features/rust:1": {
      "version": "latest"
    }
  }
}
```

## まとめ

Dev Container Featuresを使用することで、以下を実現：

### 技術的メリット
1. **設定簡素化**: 複雑なDockerfile記述不要
2. **即座開発**: 事前設定済み環境で即座に開発開始
3. **組み合わせ自由**: 複数言語・ツールの柔軟な組み合わせ
4. **保守性**: 公式メンテナンスされた設定の利用

### 開発効率向上
1. **環境統一**: チーム全体での開発環境統一
2. **学習コスト削減**: Docker詳細知識不要で環境構築
3. **時間短縮**: 環境構築時間の大幅短縮
4. **トラブル削減**: 「私の環境では動く」問題の解決

ただし、プレビュー機能である点と導入可能ツールの制限を理解した上での利用が重要。将来的な仕様変更に備えてDockerfile方式への移行準備も考慮すべき。