# Dev ContainerでTauriの開発環境を作る

> **原文**: [Dev ContainerでTauriの開発環境を作る](https://zenn.dev/hkthirano/articles/122b36a6585120)

## 概要

TauriのGUI開発環境をDev Containerで構築する方法。Desktop (Lightweight) Featureを使用してWebブラウザ上でUbuntu画面を表示し、その上でTauriアプリケーションのGUIを動作させる画期的なアプローチ。

## 技術的なポイント

### GUI表示の課題と解決
- **課題**: TauriはGUIアプリケーションのため、通常のDev Containerでは画面表示ができない
- **解決**: Desktop (Lightweight) Featureを使用してWebブラウザ上にLinuxデスクトップ環境を構築

### アーキテクチャ
```
ブラウザ (ホストマシン)
    ↓ HTTP (port 6080)
Docker Container (Ubuntu Desktop)
    ├── noVNC Server
    ├── Tauri開発環境
    │   ├── Rust
    │   ├── Node.js
    │   └── 必要な依存関係
    └── GUI Display
```

## 実装設定

### devcontainer.json
```json
{
  "name": "Ubuntu",
  "image": "mcr.microsoft.com/devcontainers/base:jammy",
  "features": {
    "ghcr.io/devcontainers/features/desktop-lite:1": {
      "version": "latest",
      "noVncVersion": "1.2.0",
      "password": "vscode",
      "webPort": "6080",
      "vncPort": "5901"
    },
    "ghcr.io/devcontainers/features/node:1": {
      "version": "18"
    },
    "ghcr.io/devcontainers/features/rust:1": {
      "version": "1.79"
    }
  },
  "postCreateCommand": "/bin/sh ./.devcontainer/setup.sh"
}
```

### 設定解説

#### Desktop Lite Feature
```json
"ghcr.io/devcontainers/features/desktop-lite:1": {
  "version": "latest",
  "noVncVersion": "1.2.0",
  "password": "vscode",
  "webPort": "6080",
  "vncPort": "5901"
}
```

- **noVncVersion**: WebブラウザでVNC接続するためのバージョン
- **password**: VNC接続パスワード（セキュリティ設定）
- **webPort**: ブラウザアクセス用ポート（通常6080）
- **vncPort**: VNC直接接続用ポート

#### 開発環境
```json
"ghcr.io/devcontainers/features/node:1": {
  "version": "18"
},
"ghcr.io/devcontainers/features/rust:1": {
  "version": "1.79"
}
```

- **Node.js 18**: フロントエンド部分の開発
- **Rust 1.79**: Tauriバックエンドの開発

### setup.sh（Tauri依存関係）
```bash
sudo apt update -y
sudo apt install -y libwebkit2gtk-4.1-dev \
  build-essential \
  curl \
  wget \
  file \
  libxdo-dev \
  libssl-dev \
  libayatana-appindicator3-dev \
  librsvg2-dev
```

### 依存関係の説明
- **libwebkit2gtk-4.1-dev**: WebViewのGTKバインディング
- **build-essential**: コンパイルツール群
- **libxdo-dev**: X11操作ライブラリ
- **libssl-dev**: SSL/TLS暗号化ライブラリ
- **libayatana-appindicator3-dev**: システムトレイ機能
- **librsvg2-dev**: SVG画像処理ライブラリ

## 使用手順

### 1. 環境構築
```bash
# Dev Container起動
# VS Code: "Reopen in Container"を選択
```

### 2. デスクトップアクセス
```
ブラウザで http://localhost:6080 にアクセス
パスワード: vscode
```

### 3. Tauriプロジェクト作成
```bash
# コンテナ内のターミナルで実行
npm create tauri-app@latest
cd tauri-app
npm install
npm run tauri dev
```

### 4. GUI確認
- ブラウザ上のUbuntuデスクトップでTauriアプリが表示される
- 通常のデスクトップアプリケーションと同様に操作可能

## 技術的メリット

### 環境隔離
- **ローカル環境保護**: ホストマシンにTauri依存関係をインストール不要
- **クリーンな開発**: プロジェクト終了時にコンテナ削除で完全クリーンアップ

### チーム開発
- **環境統一**: 全開発者が同一の開発環境を共有
- **セットアップ簡素化**: 複雑なLinux依存関係の個別インストール不要

### 柔軟性
- **OS非依存**: Windows/Mac/LinuxどこでもTauri開発可能
- **バージョン管理**: Rust/Node.jsバージョンを設定ファイルで管理

## 活用シーン

### 適用場面
- **クロスプラットフォーム開発**: 異なるOS間でのTauri開発
- **チーム開発**: 環境セットアップの標準化
- **学習・検証**: Tauriの試用・プロトタイプ開発
- **CI/CD**: 自動テスト環境での GUI テスト

### 実用的な開発ワークフロー
```bash
# 1. フロントエンド開発
npm run dev

# 2. バックエンド開発
cargo check

# 3. GUI統合テスト
npm run tauri dev

# 4. ビルド確認
npm run tauri build
```

## 注意点・制限事項

### パフォーマンス
- **ネットワーク遅延**: VNC経由のため、ローカル GUI より若干重い
- **解像度制限**: ブラウザウィンドウサイズに依存

### セキュリティ
- **VNCパスワード**: デフォルトパスワードの変更推奨
- **ポート公開**: 本番環境では適切なファイアウォール設定必要

## 拡張・カスタマイズ

### より高解像度表示
```json
"ghcr.io/devcontainers/features/desktop-lite:1": {
  "resolution": "1920x1080",
  "password": "your-secure-password"
}
```

### 追加開発ツール
```bash
# setup.shに追加
sudo apt install -y \
  firefox \
  code \
  git-gui
```

## まとめ

Dev ContainerとDesktop Lite Featureを組み合わせることで、Tauriの完全な開発環境をコンテナ化することが可能。特に以下の価値を提供：

1. **環境統一**: チーム全体でのTauri開発環境標準化
2. **クロスプラットフォーム**: OS関係なくTauri開発が可能
3. **簡単セットアップ**: 複雑なLinux依存関係を自動解決
4. **隔離環境**: ローカル環境を汚さない安全な開発

GUI アプリケーション開発における Dev Container の新しい可能性を示す革新的なアプローチ。