# Bun.js + Docker + VSCodeで爆速ローカル開発環境構築

## 概要
Bun.js（高速なJavaScriptランタイム）をDocker + VSCodeのDevContainer機能で構築する方法を紹介。ホスト環境に何もインストールせずにBun.js開発を始められるテンプレート。

## 著者
- **ユーザー**: MoonDevLog
- **記事URL**: https://qiita.com/MoonDevLog/items/3a30484557d5d42f5834

## 前提条件
### 必要なツール
- Visual Studio Code
- Docker Desktop
- VSCode拡張機能: Dev Containers

## フォルダ構成
```
bun-docker-dev/
└── .devcontainer/
    ├── Dockerfile
    └── devcontainer.json
```

## 設定ファイル

### .devcontainer/Dockerfile
```dockerfile
FROM oven/bun:latest

# Gitをインストール（GitHub連携やバージョン管理向け）
RUN apt update && apt install -y git

WORKDIR /app
CMD ["sleep", "infinity"]
```

### .devcontainer/devcontainer.json
```json
{
  "name": "bun-dev-env",
  "build": {
    "dockerfile": "Dockerfile"
  },
  "customizations": {
    "vscode": {
      "settings": {
        "terminal.integrated.defaultProfile.linux": "bash"
      },
      "extensions": [
        "esbenp.prettier-vscode"
      ]
    }
  },
  "forwardPorts": [],
  "postCreateCommand": ""
}
```

## 手順

1. **フォルダをVSCodeで開く**
   - `bun-docker-dev/`フォルダをVSCodeで開く

2. **開発コンテナ起動**
   - コマンドパレット（`Ctrl + Shift + P`）を開く
   - `Dev Containers: Reopen in Container`を実行

3. **Bunプロジェクト初期化**
   ```bash
   bun init
   ```
   - プロンプトに従ってプロジェクト名やエントリーポイントを指定

4. **実行確認**
   ```bash
   bun run src/index.ts
   ```

## 最終的な構成
```
bun-docker-dev/
├── .devcontainer/
│   ├── Dockerfile
│   └── devcontainer.json
├── node_modules/
├── src/
│   └── index.ts
├── .gitignore
├── bun.lockb
├── package.json
├── README.md
└── tsconfig.json
```

## 便利なポイント
- Gitがインストール済みでバージョン管理が即座に可能
- ホスト環境を汚さず数分で準備完了
- プロジェクトのバージョン管理も即座に開始可能

## まとめ
- Bun.jsは速くてシンプルな新時代のJavaScriptランタイム
- Docker + VSCode DevContainerで環境構築が簡単
- サンプルリポジトリ: https://github.com/Mangetu/BunDevEnvForBlog

## タグ
#BunJS #DevContainer #Docker #VSCode #JavaScript #環境構築
