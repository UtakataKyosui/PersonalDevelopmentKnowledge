# Docker Composeな開発環境にちょい足し3分で作るVSCode devcontainer

> **原文**: [Docker Compose な開発環境にちょい足し3分で作るVSCode devcontainer](https://zenn.dev/shunjuio/articles/9cffc8d14c6684)

## 概要

既にDocker Composeを利用している開発環境に、3分でVSCode devcontainerを構築する方法を紹介。「Existing Docker Compose (Extend)」方式を使い、既存環境を活かしつつdevcontainerの恩恵を受ける実践的アプローチ。

## 前提条件

### 既存のDocker Compose環境
Ruby on Rails + PostgreSQLの構成例：

#### docker-compose.yml
```yaml
version: '3'
services:
  app:
    build:
      context: .
      dockerfile: docker/app/Dockerfile
    env_file: .env
    ports:
      - 3000:3000
    volumes:
      - .:/workspace:cached
      - bundle-volume:/workspace/vendor/bundle
      - node-module-volume:/workspace/node_modules
    depends_on:
      - postgres
    command: 'bin/rails s'
  
  postgres:
    image: postgres:14
    restart: unless-stopped
    volumes:
      - postgres-volume:/var/lib/postgresql/data
    environment:
      POSTGRES_USER: pg
      POSTGRES_PASSWORD: password
    ports:
      - 5432:5432

volumes:
  postgres-volume:
  bundle-volume:
  node-module-volume:
```

#### Dockerfile
```dockerfile
FROM ruby:2.7.6-bullseye

ENV BUNDLE_APP_CONFIG=/workspace/.bundle

# Install nodejs, yarn
RUN curl -fsSL https://deb.nodesource.com/setup_16.x | bash -
RUN apt-get update && apt-get install -y nodejs
RUN npm install -g yarn

RUN gem install bundler -v 2.3.13

WORKDIR /workspace
```

## devcontainer化の手順

### 必要なファイル（2つのみ）

#### 1. .devcontainer/devcontainer.json
```json
{
  "dockerComposeFile": ["../docker-compose.yml", "docker-compose.yml"],
  "service": "app",
  "workspaceFolder": "/workspace"
}
```

#### 2. .devcontainer/docker-compose.yml
```yaml
version: '3'
services:
  app:
    command: sleep infinity
```

### 起動方法
1. VSCodeでプロジェクトディレクトリを開く
2. 「Reopen folder in Container」を実行

## 設定の解説

### devcontainer.json
- **dockerComposeFile**: 使用するdocker-compose.ymlファイルのパス（複数指定可能、後の設定で上書き）
- **service**: devcontainerとして起動するコンテナ名
- **workspaceFolder**: devcontainerで展開するワークスペースのルート

### .devcontainer/docker-compose.yml
- **command: sleep infinity**: 既存の`command: 'bin/rails s'`をオーバーライド
- devcontainerの要件として「起動しているコンテナにアタッチできればよい」ため、プロセスを起動し続けるだけで十分

## devcontainer構築の方式比較

### 主要な構築方法
1. **imageを利用する**
2. **devcontainer用のDockerfileを用意する**
3. **devcontainer用のdocker-compose.ymlを用意する**
4. **既存のDocker Composeをオーバーライドする** ← 今回の方式
5. **kubernetes上のcontainerにアタッチする**

## 既存Docker Composeオーバーライド方式のメリット

### 開発効率
- **既存知識の活用**: Docker Composeの知識をそのまま使い回せる
- **環境の使い回し**: 既に作成しているdocker-compose.ymlを活用
- **高速化との両立**: named volumeを使った高速化とIntellisense/Go to Definitionが両立

### チーム開発の配慮
- **VSCode特化しない**: VSCode以外のエディタユーザーも従来通り開発可能
- **混在環境対応**: ファイル操作不要でプログラム起動する場合は`docker compose up`で従来通り

### 技術的メリット
- **パフォーマンス最適化**: named volumeによるDockerの高速化
- **コード参照機能**: HostOSから隠蔽されたファイルもdevcontainer内から参照可能

## 実用例・サンプル

### 対応言語・フレームワーク
- **Ruby**: Rails, Sinatra
- **Node.js**: Apollo Server, Nuxt
- **Python**: FastAPI

### GitHubサンプルリポジトリ
- rails-devcontainer
- apollo-server-devcontainer
- nuxt-devcontainer
- fastapi-devcontainer
- sinatra-devcontainer

## 拡張設定（オプション）

### より高機能な設定
```json
{
  "dockerComposeFile": ["../docker-compose.yml", "docker-compose.yml"],
  "service": "app",
  "workspaceFolder": "/workspace",
  "customizations": {
    "vscode": {
      "extensions": ["ms-python.python"],
      "settings": {
        "terminal.integrated.shell.linux": "/bin/bash"
      }
    }
  },
  "postCreateCommand": "bundle install"
}
```

## 注意点・Tips

### パフォーマンス最適化
- **named volume活用**: `vendor/bundle`, `node_modules`をnamed volume化
- **virtiofsの課題**: M1 MacのDocker Desktop高速化オプションは実験的機能で不安定な場合あり

### 開発ワークフロー
- **VSCodeユーザー**: devcontainerで最高の開発体験
- **その他エディタ**: 従来通りDocker Compose使用
- **チーム混在環境**: 各自の好みに応じて選択可能

## まとめ

既存のDocker Compose環境を活かしつつ、最小限の設定（2ファイル）でdevcontainerを導入可能。VSCode特化せず、チーム全体の開発体験を損なわない現実的なアプローチ。導入コストが低く、段階的な移行も可能。