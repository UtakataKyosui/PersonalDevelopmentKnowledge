# TauriによるWeb & Desktop & Android開発の環境

## メタデータ
- **作成日時**: 2025-02-12T06:38:00.000Z
- **最終更新**: 2025-07-01T20:03:00.000Z
- **タグ**: #Tauri #Desktop #Android #Rust #WebDevelopment
- **移植元**: Notion
- **移植日**: 2025-07-02

## 概要

Tauriを使用してWeb、Desktop、Androidアプリケーションを開発するための環境構築と開発手法についてまとめたページです。

## Tauri 主要リソース

### Docker環境
- [nailyudha/tauri](https://hub.docker.com/r/nailyudha/tauri) - 公式Dockerイメージ
- [ivangabriele/docker-tauri](https://github.com/ivangabriele/docker-tauri) - GitHubリポジトリ
- [nailyudha/tauri:2.0-2.0.0](https://hubgw.docker.com/layers/nailyudha/tauri/2.0-2.0.0/images/sha256-a4669a7983fd648b30e4c3b7ea4a1357105fff56aa0fcbfffe009cf3057654f2) - 特定バージョン

### 学習リソース
- [Tauri でデスクトップアプリ開発を始める](https://developer.mamezou-tech.com/blogs/2022/07/08/writing-app-with-tauri/)
- [セキュリティ強化されたTauriアプリケーションの特徴と実装](https://www.issoh.co.jp/tech/details/4114/)

### 実践例・チュートリアル
- [Build a Custom Docker Desktop App Using Tauri 2.0 and Rust | Step-by-Step Guide](https://www.youtube.com/watch?v=KOKFCq4GR4o)
- [TauriをDockerを使ってなるべく簡単に始める](https://www.tunamaguro.dev/articles/20230507-Tauri%E3%82%92Docker%E3%82%92%E4%BD%BF%E3%81%A3%E3%81%A6%E3%81%AA%E3%82%8B%E3%81%B9%E3%81%8F%E7%B0%A1%E5%8D%98%E3%81%AB%E5%A7%8B%E3%82%81%E3%82%8B/)

## Docker環境でのTauri開発

### 基本的なDockerfile例
```dockerfile
FROM nailyudha/tauri:2.0-2.0.0

WORKDIR /app
COPY . .

# 依存関係のインストール
RUN npm install

# Tauriアプリのビルド
RUN npm run tauri build
```

### Docker Composeを使った開発環境
```yaml
version: '3.8'
services:
  tauri-dev:
    image: nailyudha/tauri:latest
    volumes:
      - .:/app
      - /tmp/.X11-unix:/tmp/.X11-unix:rw
    environment:
      - DISPLAY=${DISPLAY}
    working_dir: /app
    command: npm run tauri dev
```

### WSLg環境での設定
- [Docker で Tauri の環境を構築する(yew,WSLg)](https://qiita.com/Ritz/items/883337f711a48663cf64)
- [Docker で Tauri の環境構築する](https://qiita.com/Ritz/items/ecb4bc2d55a0d6967e6e)

## DevContainer環境

### 基本設定
```json
{
  "name": "Tauri Development",
  "image": "nailyudha/tauri:2.0-2.0.0",
  "features": {
    "ghcr.io/devcontainers/features/rust:1": {},
    "ghcr.io/devcontainers/features/node:1": {
      "version": "18"
    }
  },
  "customizations": {
    "vscode": {
      "extensions": [
        "tauri-apps.tauri-vscode",
        "rust-lang.rust-analyzer"
      ]
    }
  }
}
```

### 参考記事
- [Dev Container で Tauri を用いた GUI 開発](https://pc.atsuhiro-me.net/entry/2023/04/12/040513)
- [Docker の node コンテナで Tauri アプリに入門する](https://mikoto2000.blogspot.com/2022/03/docker-node-tauri.html)

## Android開発

### 環境設定
- [Dev Container for Tauri + Android](https://www.youtube.com/watch?v=X3EThnruPA4)
- [Tauri 2.0でAndroidの位置情報取得機能を使ってみた](https://puripuri2100.hatenablog.com/entry/2024/10/03/154151)

### 必要なツール
- Android Studio
- Android SDK
- NDK (Native Development Kit)
- Java Development Kit (JDK)

### タグ分類

#### Docker関連
- Docker環境でのTauri開発
- Dockerイメージの活用
- コンテナ化のベストプラクティス

#### Android Studio関連
- Android開発環境の構築
- エミュレータの設定
- デバッグ手法

#### 開発手法
- クロスプラットフォーム開発
- ビルドプロセスの最適化

#### GitHub関連
- CI/CDパイプライン
- リポジトリ管理

#### Tips
- トラブルシューティング
- パフォーマンス最適化
- セキュリティ強化

## プロジェクト構造例

```
tauri-app/
├── src-tauri/          # Rustバックエンド
│   ├── src/
│   │   └── main.rs
│   ├── Cargo.toml
│   └── tauri.conf.json
├── src/                # フロントエンド (React/Vue/etc)
│   ├── components/
│   ├── pages/
│   └── main.tsx
├── package.json
└── .devcontainer/
    └── devcontainer.json
```

## Tauri設定例

### tauri.conf.json
```json
{
  "build": {
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build",
    "devPath": "http://localhost:1420",
    "distDir": "../dist"
  },
  "package": {
    "productName": "my-tauri-app",
    "version": "0.0.0"
  },
  "tauri": {
    "allowlist": {
      "all": false,
      "shell": {
        "all": false,
        "open": true
      }
    },
    "bundle": {
      "active": true,
      "targets": "all"
    }
  }
}
```

## 関連リンク
- [[Tauri-React]] - Tauri と React の連携
- [[DevContainer-CLI]] - DevContainer CLIの活用
- [[Rust-DevContainer-WebAssembly開発]] - Rust開発環境

## ノート
- このページはNotionから移植されました
- Tauri 2.0では多くの新機能が追加されているため、最新ドキュメントの確認を推奨
- Android開発には追加の環境設定が必要です
- Docker環境を使用することで、開発環境の一貫性を保つことができます
