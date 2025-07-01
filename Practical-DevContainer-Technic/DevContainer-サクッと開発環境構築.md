# DevContainerでサクッと開発環境を構築しよう！

> **原文**: [【超便利】DevContainerでサクッと開発環境を構築しよう！](https://zenn.dev/secondselection/articles/how_to_devcontainer)

## 概要

Dev Containerの基本から実用例まで、新規構築・既存Dockerfile活用・Docker Compose連携の3つのパターンを網羅的に解説。Zenn執筆環境からLaravel+MySQL構成まで、実際のプロジェクトで使える実践的な設定例を豊富に紹介。

## Dev Containerとは

### 基本特徴
- **VSCode拡張機能**: Dockerコンテナを利用した開発環境構築
- **環境統一**: VSCodeとDockerがあれば誰でも同じ環境を構築可能
- **コード化**: 構築時の差異が発生しづらい
- **拡張機能統一**: VSCodeの拡張機能もチーム内で統一可能

### 主要メリット
1. **ローカル環境を汚さない**
2. **環境構築をコード化できる**
3. **lintなどの拡張機能をチームで統一**
4. **Dockerを使っているプロジェクトであれば容易に使える**

## パターン1: 新規構築（Zenn執筆環境）

### 最終的な設定ファイル
```json
{
  "name": "zenn",
  "image": "mcr.microsoft.com/devcontainers/base:jammy",
  "features": {
    "ghcr.io/devcontainers/features/node:1": {}
  },
  "customizations": {
    "vscode": {
      "extensions": [
        "taichi.vscode-textlint",
        "DavidAnson.vscode-markdownlint",
        "hediet.vscode-drawio",
        "bierner.markdown-mermaid"
      ],
      "settings": {}
    }
  },
  "postCreateCommand": "yarn install"
}
```

### 構築できる環境
- **Zenn CLI**: ターミナルで実行可能
- **校正機能**: VSCode上でのtextlint
- **図編集**: drawio・mermaidのプレビュー
- **Markdown**: markdownlintによる構文チェック

### GUI構築手順
1. **VS Code左下「><」** を選択
2. **Open Container Configuration File** を選択
3. **イメージ選択**: "Ubuntu"のイメージ
4. **タグ選択**: "jammy"タグ
5. **追加ツール**: "Nodejs"を選択
6. **設定ファイル作成**: "OK" → "Keep Defaults"
7. **環境起動**: 右下の "Reopen in Container" をクリック

### パッケージインストール
```bash
# 必要なパッケージをインストール
yarn add zenn-cli
yarn add --dev markdownlint-cli2
yarn add --dev textlint
# textlint-rule-xxxも同時に追加
```

### 自動化設定
```json
{
  "postCreateCommand": "yarn install"
}
```

## パターン2: 既存Dockerfileを利用

### 設定変更のポイント
```json
{
  // 変更前
  "image": "mcr.microsoft.com/devcontainers/base:jammy",
  "features": {
    "ghcr.io/devcontainers/features/node:1": {}
  },
  
  // 変更後
  "build": {
    "dockerfile": "../Dockerfile"
  }
}
```

### フォルダ構成
```
project-root/
├── .devcontainer/
│   └── devcontainer.json
└── Dockerfile（既存ファイル）
```

## パターン3: 既存docker-compose.yml利用

### 設定変更のポイント
```json
{
  // 変更前
  "image": "mcr.microsoft.com/devcontainers/base:jammy",
  "features": {
    "ghcr.io/devcontainers/features/node:1": {}
  },
  
  // 変更後
  "dockerComposeFile": [
    "../docker-compose.yml"
  ],
  "service": "サービス名",
  "workspaceFolder": "フォルダパス"
}
```

### フォルダ構成
```
project-root/
├── .devcontainer/
│   └── devcontainer.json
└── docker-compose.yml（既存ファイル）
```

## 実用例1: Python+Poetry環境

### devcontainer.json
```json
{
  "name": "poetry3-poetry-pyenv",
  "build": {
    "dockerfile": "Dockerfile"
  },
  "customizations": {
    "vscode": {
      "extensions": [
        "ms-python.python",
        "njpwerner.autodocstring"
      ]
    }
  }
}
```

### 主要な特徴
- **pyenv**: Python バージョン管理
- **poetry**: 依存関係管理
- **仮想環境**: プロジェクト内に.venv作成

## 実用例2: Laravel+MySQL環境

### devcontainer.json
```json
{
  "name": "PHP",
  "dockerComposeFile": [
    "../docker-compose.yml"
  ],
  "service": "webapp",
  "workspaceFolder": "/var/www/html"
}
```

### Laravel環境の特徴
- **PHP 8.3**: 最新のPHP環境
- **Apache**: Webサーバー
- **MySQL 8.0**: データベース
- **Composer**: PHP依存関係管理
- **Node.js & Yarn**: フロントエンド用
- **Xdebug**: デバッグ機能

## 拡張機能のID確認方法

### VS Code拡張機能管理
1. **拡張機能タブ**を開く
2. **対象拡張機能**を右クリック
3. **「拡張機能IDをコピー」**を選択
4. **devcontainer.json**の`extensions`配列に追加

### 自動インストールの仕組み
```json
{
  "customizations": {
    "vscode": {
      "extensions": [
        "ms-python.python",              // Python
        "ms-vscode.vscode-typescript-next", // TypeScript
        "esbenp.prettier-vscode",         // Prettier
        "dbaeumer.vscode-eslint"           // ESLint
      ]
    }
  }
}
```

## 環境再構築の手順

### 設定変更後の反映
1. **devcontainer.json**を保存
2. **右下に表示される「Rebuild」**をクリック
3. **コンテナの再ビルド**が実行される
4. **新しい設定で環境起動**

### 手動再ビルド
```
VS Code Command Palette (⌘+Shift+P)
> Dev Containers: Rebuild Container
```

## 設定パターンの比較

### 新規構築 vs 既存環境活用

| 項目 | 新規構築 | 既存Dockerfile | 既存Docker Compose |
|------|----------|---------------|---------------------|
| **学習コスト** | 低 | 中 | 中 |
| **カスタマイズ性** | 中 | 高 | 高 |
| **既存資産活用** | なし | あり | あり |
| **複数サービス** | 困難 | 困難 | 容易 |
| **チーム導入** | 容易 | 中 | 中 |

## 実践的なTips

### パフォーマンス最適化
```json
{
  "mounts": [
    "source=project-node_modules,target=${containerWorkspaceFolder}/node_modules,type=volume"
  ]
}
```

### 環境変数の設定
```json
{
  "containerEnv": {
    "NODE_ENV": "development",
    "DATABASE_URL": "postgresql://user:pass@db:5432/mydb"
  }
}
```

### ポートフォワード
```json
{
  "forwardPorts": [3000, 5432],
  "portsAttributes": {
    "3000": {
      "label": "Application",
      "onAutoForward": "notify"
    }
  }
}
```

## まとめ

Dev Containerの3つのアプローチにより、様々な開発環境ニーズに対応可能：

### 新規プロジェクト
- **GUI設定**: 初心者にも優しい
- **テンプレート活用**: 公式イメージで迅速構築
- **段階的カスタマイズ**: 必要に応じて拡張

### 既存プロジェクト
- **Dockerfile活用**: 既存の投資を無駄にしない
- **Docker Compose連携**: 複数サービス環境をそのまま活用
- **段階的移行**: リスクを抑えた導入

### チーム開発
- **環境統一**: 「私の環境では動く」問題を解決
- **拡張機能統一**: linter・formatter等のルール統一
- **新人教育**: セットアップ時間を劇的短縮

特に「コンテナでの環境構築の便利さ」と「チーム開発での拡張機能統一」の価値は計り知れない。既存のDocker資産があれば容易に導入できる点も大きなメリット。