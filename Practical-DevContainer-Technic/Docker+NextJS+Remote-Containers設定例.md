# Docker + Next.js + Remote-Containers設定例

## 概要
Docker + Next.js + VSCodeのRemote-Containers（DevContainer）を組み合わせた開発環境の設定ファイル集。実用的な設定例を提供する構成重視の記事。

## 著者
- **ユーザー**: P-man_Brown
- **記事URL**: https://qiita.com/P-man_Brown/items/de80c0fdf75c990959e8

## 設定ファイル構成

### .devcontainer/devcontainer.json
```json
{
  "name": "myapp-frontend",
  "dockerComposeFile": [
    "../docker-compose.yml",
    "./docker-compose.extend.yml"
  ],
  "service": "frontend",
  "workspaceFolder": "/myapp-frontend",
  "customizations": {
    "vscode": {
      "extensions": [
        "EditorConfig.EditorConfig",
        "dbaeumer.vscode-eslint",
        "esbenp.prettier-vscode",
        "VisualStudioExptTeam.vscodeintellicode",
        "arcanis.vscode-zipfs",
        "bradlc.vscode-tailwindcss"
      ]
    }
  }
}
```

### .devcontainer/Dockerfile
```dockerfile
FROM node:16.15.0

ENV TZ=Asia/Tokyo
ARG USERNAME=node

RUN corepack enable npm

# コマンド履歴の永続化設定
RUN SNIPPET="export PROMPT_COMMAND='history -a' && export HISTFILE=/commandhistory/.bash_history" \
    && mkdir /commandhistory \
    && touch /commandhistory/.bash_history \
    && chown -R ${USERNAME} /commandhistory \
    && echo ${SNIPPET} >> "/home/${USERNAME}/.bashrc"

USER ${USERNAME}

# VSCode拡張機能用ディレクトリ作成
RUN mkdir -p /home/${USERNAME}/.vscode-server/extensions

WORKDIR /myapp-frontend
EXPOSE 3000
```

### .devcontainer/docker-compose.extend.yml
```yaml
version: '3.9'

services:
  frontend:
    build:
      context: .
      dockerfile: ./.devcontainer/Dockerfile
    volumes:
      - sources:/myapp-frontend
      - bashlog:/bashlog
      - vscode-extensions:/home/node/.vscode-server/extensions
    command: /bin/sh -c "while sleep 1000; do :; done"

volumes:
  sources:
  bashlog:
  vscode-extensions:
```

### ルートディレクトリの設定

#### /Dockerfile
```dockerfile
FROM node:16.15.0

ENV TZ=Asia/Tokyo
WORKDIR /myapp-frontend

RUN corepack enable npm
USER node
EXPOSE 3000

CMD ["yarn", "dev"]
```

#### /docker-compose.yml
```yaml
version: '3.9'

services:
  frontend:
    build: .
    volumes:
      - .:/myapp-frontend
    environment:
      NODE_ENV: development
    ports:
      - 3000:3000
      - 9229:9229  # デバッグポート
```

## 設定の特徴

### 1. マルチDockerファイル構成
- **本番用**: `/Dockerfile`
- **開発用**: `.devcontainer/Dockerfile`
- 環境別の要件に応じた使い分け

### 2. 拡張機能の統一
DevContainer起動時に自動インストールされる拡張機能：
- EditorConfig
- ESLint
- Prettier
- IntelliCode
- Zipfs（Yarn PnP対応）
- Tailwind CSS

### 3. 開発効率向上機能
- **コマンド履歴永続化**: `/commandhistory`ボリューム
- **VSCode拡張機能永続化**: 再起動時の拡張機能再インストール回避
- **デバッグポート**: 9229ポートでNode.jsデバッグ対応

### 4. ボリューム管理
- **sources**: ソースコード用
- **bashlog**: コマンド履歴用
- **vscode-extensions**: 拡張機能用

## 使用方法
1. プロジェクトルートに上記ファイルを配置
2. VSCodeでプロジェクトを開く
3. `Remote-Containers: Reopen in Container`を実行

## タグ
#DevContainer #Docker #NextJS #VSCode #RemoteContainers #Node.js #環境構築
