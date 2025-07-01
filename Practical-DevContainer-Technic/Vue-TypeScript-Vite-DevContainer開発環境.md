# Vue.js + TypeScript + Vite + VSCode DevContainers で開発環境を作る

> **原文**: [Vue.js + TypeScript + Vite + VSCode DevContainers で開発環境を作る](https://zenn.dev/mk663/articles/6861978fa091a3)

## 概要

Vue.js + TypeScript + Viteの最新構成をDev Containersで構築する実践的なガイド。ESLint・Prettier・Vitestを含む本格的なフロントエンド開発環境の構築手順を詳細に解説。

## プロジェクト仕様

### 技術スタック
- **Vue.js**: 最新版のプログレッシブJavaScriptフレームワーク
- **TypeScript**: 型安全性を提供
- **Vite**: 高速ビルドツール
- **Vue Router**: SPAルーティング
- **Pinia**: 状態管理
- **Vitest**: ユニットテスト
- **Playwright**: E2Eテスト
- **ESLint**: コード品質管理
- **Prettier**: コードフォーマッター

### 動作環境
- **MacOS**: Intel版
- **VS Code**: v1.97.2
- **Node.js**: v22.14.0
- **Docker**: v27.4.0
- **Docker Desktop**: v4.37.2

## プロジェクト初期構築

### Vue.jsプロジェクト作成
```bash
npm create vue@latest

# 対話的設定
✔ Project name: … アプリ名
✔ Add TypeScript? … Yes
✔ Add JSX Support? … No
✔ Add Vue Router for Single Page Application development? … Yes
✔ Add Pinia for state management? … Yes
✔ Add Vitest for Unit Testing? … Yes
✔ Add an End-to-End Testing Solution? › Playwright
✔ Add ESLint for code quality? › Yes
✔ Add Prettier for code formatting? … Yes
```

### 初期セットアップ
```bash
cd アプリ名
npm install
npm run format
npm run dev
```

## Dev Container設定

### ディレクトリ構造
```
project-root/
├── .devcontainer/
│   ├── devcontainer.json
│   └── Dockerfile
├── .vscode/
│   ├── extensions.json
│   └── settings.json
├── src/
├── package.json
└── vite.config.ts
```

### .vscode/extensions.json
```json
{
  "recommendations": ["ms-vscode-remote.remote-containers"]
}
```

**目的**: チーム開発で推奨するVS Code拡張機能を統一

### .vscode/settings.json
```json
{
  "explorer.fileNesting.enabled": true,
  "explorer.fileNesting.patterns": {
    "tsconfig.json": "tsconfig.*.json, env.d.ts",
    "vite.config.*": "jsconfig*, vitest.config.*, cypress.config.*, playwright.config.*",
    "package.json": "package-lock.json, pnpm*, .yarnrc*, yarn*, .eslint*, eslint*, .prettier*, prettier*, .editorconfig"
  },
  "editor.codeActionsOnSave": {
    "source.fixAll": "explicit"
  },
  "editor.formatOnPaste": true,
  "editor.formatOnSave": true,
  "editor.formatOnType": true,
  "[typescript]": {
    "editor.defaultFormatter": "esbenp.prettier-vscode",
    "editor.tabSize": 2,
    "editor.insertSpaces": true
  },
  "[vue]": {
    "editor.defaultFormatter": "esbenp.prettier-vscode",
    "editor.tabSize": 2,
    "editor.insertSpaces": true
  },
  "[css]": {
    "editor.tabSize": 2
  }
}
```

**主要設定**:
- **ファイルネスティング**: 関連ファイルの階層表示
- **自動フォーマット**: 保存時・ペースト時・入力時の自動整形
- **タブサイズ統一**: TypeScript/Vue/CSSで2スペース統一

### .devcontainer/devcontainer.json
```json
{
  "name": "アプリ名",
  "build": {
    "dockerfile": "Dockerfile",
    "context": "."
  },
  "remoteUser": "node",
  "customizations": {
    "vscode": {
      "settings": {
        "[css][html][javascript][typescript][vue]": {
          "editor.defaultFormatter": "esbenp.prettier-vscode"
        },
        "[json][jsonc]": {
          "editor.defaultFormatter": "vscode.css-language-features"
        },
        "github.copilot-chat.enable": true
      },
      "extensions": [
        "Vue.volar",
        "vitest.explorer",
        "ms-playwright.playwright",
        "dbaeumer.vscode-eslint",
        "EditorConfig.EditorConfig",
        "esbenp.prettier-vscode",
        "eamodio.gitlens",
        "ms-azuretools.vscode-docker",
        "GitHub.copilot",
        "GitHub.copilot-chat"
      ]
    }
  }
}
```

**重要な拡張機能**:
- **Vue.volar**: Vue 3公式Language Server
- **vitest.explorer**: テストエクスプローラー統合
- **ms-playwright.playwright**: E2Eテストサポート
- **GitHub.copilot**: AI コード支援

### .devcontainer/Dockerfile
```dockerfile
FROM node:lts-slim

# 基本ツールのインストール
RUN apt-get update && \
    apt-get install -y git locales sudo vim-tiny && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# 日本語ロケール設定
RUN sed -i -E 's/# (ja_JP.UTF-8)/\1/' /etc/locale.gen && locale-gen

# nodeユーザーにsudo権限付与
RUN echo 'node ALL=(ALL) NOPASSWD: ALL' >> /etc/sudoers

# 環境変数設定
ENV LANG ja_JP.UTF-8
ENV TZ Asia/Tokyo
ENV GENERATE_SOURCEMAP=false

USER node
```

**設定のポイント**:
- **ベースイメージ**: `node:lts-slim`で軽量かつ安定
- **日本語対応**: ロケール設定で文字化け防止
- **開発効率**: sudo権限でパッケージインストール可能
- **Source Map無効**: 本番ビルドサイズ削減

## Vite設定の調整

### vite.config.ts
```typescript
import { fileURLToPath, URL } from 'node:url'
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueDevTools from 'vite-plugin-vue-devtools'

export default defineConfig({
  plugins: [
    vue(),
    vueDevTools(),
  ],
  // Dev Container用ネットワーク設定
  server: {
    host: true,
    port: 5173,
    cors: {
      origin: ['http://127.0.0.1:5173', 'http://127.0.0.1:5173/__devtools__/']
    }
  },
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url))
    },
  },
})
```

**重要な設定**:
- **host: true**: コンテナ外からのアクセスを許可
- **port: 5173**: Viteデフォルトポート
- **CORS設定**: Dev ToolsとのCORS問題を解決

## 開発ワークフロー

### 1. 環境起動
```bash
# VS Codeでプロジェクトを開く
code .

# 右下の通知で「Reopen in Container」を選択
# または⌘+Shift+P -> "Dev Containers: Reopen in Container"
```

### 2. パッケージインストール
```bash
# コンテナ内で実行
npm install
```

### 3. 開発サーバー起動
```bash
npm run dev
```

### 4. 各種開発コマンド
```bash
# TypeScript型チェック
npm run type-check

# リンターチェック
npm run lint

# フォーマット
npm run format

# ユニットテスト
npm run test:unit

# E2Eテスト
npm run test:e2e

# ビルド
npm run build
```

## ファイルマウントの仕組み

### Dev Containersのマウント
公式ドキュメントによると：
> Workspace files are mounted from the local file system or copied or cloned into the container.

**実際の動作**:
- **ローカルファイル**: ホストからコンテナにマウント
- **リアルタイム同期**: ファイル変更が即座に反映
- **拡張機能**: コンテナ内で動作、完全なアクセス権限

### 確認方法
```bash
# コンテナ内でマウント状況確認
df -h
mount | grep workspace
```

## トラブルシューティング

### よくある問題と解決策

#### 1. Viteサーバーにアクセスできない
**問題**: `npm run dev`実行後、ブラウザでアクセスできない
**解決**: `vite.config.ts`の`server.host: true`設定を確認

#### 2. ホットリロードが動作しない
**問題**: ファイル変更がブラウザに反映されない
**解決**: Viteの設定とファイルマウントを確認

#### 3. TypeScriptエラーが表示されない
**問題**: VS CodeでTypeScriptエラーが表示されない
**解決**: Vue.volar拡張機能がインストール・有効化されているか確認

## パフォーマンス最適化

### 高速化のポイント
```dockerfile
# node_modulesのキャッシュ最適化
FROM node:lts-slim

# パッケージキャッシュの活用
COPY package*.json ./
RUN npm ci --only=production

COPY . .
```

### 開発効率向上
```json
// package.json scripts最適化
{
  "scripts": {
    "dev": "vite --host",
    "dev:debug": "vite --host --debug",
    "type-check:watch": "vue-tsc --noEmit --watch"
  }
}
```

## チーム開発での活用

### 統一された開発環境
- **Node.jsバージョン**: Dockerfileで固定
- **拡張機能**: devcontainer.jsonで統一
- **設定**: .vscode/settings.jsonで標準化
- **コード品質**: ESLint/Prettierルール統一

### GitHubリポジトリテンプレート
作成されたリポジトリ例：
[vue-vite-devcontainer](https://github.com/mk663379/vue-vite-devcontainer)

### 新規メンバーのオンボーディング
```bash
# 1. リポジトリクローン
git clone [repository-url]
cd project-name

# 2. VS Codeで開く
code .

# 3. コンテナで再オープン
# (VS Codeの通知から選択)

# 4. 開発開始
npm run dev
```

## 実用的な拡張設定

### 追加推奨拡張機能
```json
{
  "customizations": {
    "vscode": {
      "extensions": [
        // 既存の拡張機能...
        "bradlc.vscode-tailwindcss",        // Tailwind CSS
        "formulahendry.auto-rename-tag",    // HTMLタグ自動リネーム
        "christian-kohler.path-intellisense", // パス補完
        "ms-vscode.vscode-typescript-next"   // TypeScript最新機能
      ]
    }
  }
}
```

### デバッグ設定
```json
// .vscode/launch.json
{
  "version": "0.2.0",
  "configurations": [
    {
      "name": "Launch Chrome",
      "request": "launch",
      "type": "chrome",
      "url": "http://localhost:5173",
      "webRoot": "${workspaceFolder}/src"
    }
  ]
}
```

## まとめ

Vue.js + TypeScript + Vite + Dev Containersの組み合わせにより、以下を実現：

### 技術的メリット
1. **環境統一**: チーム全体で同一の開発環境
2. **高速開発**: Viteによる高速ビルド・HMR
3. **型安全性**: TypeScriptによる堅牢な開発
4. **コード品質**: ESLint・Prettierによる統一的なコード品質

### 開発効率向上
1. **ゼロセットアップ**: Dev Containerで即座に開発開始
2. **拡張機能統一**: VS Code拡張機能の自動インストール
3. **設定標準化**: フォーマット・リントルールの統一
4. **テスト環境**: Vitest・Playwrightの統合環境

### 実用性
1. **本格運用**: 商用プロジェクトレベルの設定
2. **学習効率**: Vue.js学習の環境構築不要
3. **プロトタイプ**: 新機能検証の迅速な環境構築
4. **メンテナンス**: 設定のバージョン管理と共有

特にフロントエンド開発における「環境構築の複雑さ」を Dev Container で解決し、開発そのものに集中できる環境を提供する実践的なアプローチ。