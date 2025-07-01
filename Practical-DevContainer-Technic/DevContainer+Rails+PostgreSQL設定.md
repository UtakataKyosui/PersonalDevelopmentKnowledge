# DevContainer + Rails + PostgreSQL設定ガイド

## 概要
VS CodeのDev Containers拡張機能を使用したRails + PostgreSQL開発環境の構築手順。チーム開発での環境統一とローカル環境の汚染防止を目的とした詳細なセットアップガイド。

## 著者
- **ユーザー**: ikotome
- **記事URL**: https://qiita.com/ikotome/items/8b26c250adfb5d087022

## DevContainerのメリット
1. **環境再現性**: コンテナ起動だけで同じ開発環境を再現
2. **拡張機能統一**: VS Codeの拡張機能をチーム間で統一
3. **環境保護**: ローカル環境が汚れない

## 構築手順

### 1. VS Code拡張機能インストール
- `Dev Containers`拡張機能をインストール

### 2. プロジェクトディレクトリ作成
```bash
mkdir [ディレクトリ名] && cd [ディレクトリ名] && code .
```

VS Codeでシェルコマンドをインストール：
- コマンドパレット（`shift + ⌘ + P`）
- `shell command`と入力
- `シェルコマンド : PATH内にcode-insidersコマンドをインストール`を選択

### 3. 必要ファイル作成

#### devcontainer.json作成
```bash
mkdir .devcontainer && cd .devcontainer && touch devcontainer.json
```

```json
{
  "name": "Rails Devcontainer",
  "dockerComposeFile": [
    "docker-compose.yml"
  ],
  "service": "web",
  "workspaceFolder": "/workspaces/rails_app",
  "customizations": {
    "vscode": {
      "extensions": [
        "Shopify.ruby-lsp",
        "misogi.ruby-rubocop",
        "ms-azuretools.vscode-docker"
      ]
    }
  }
}
```

#### docker-compose.yml作成
```yaml
services:
  db:
    image: postgres
    env_file:
      - ../.env
    volumes:
      - .:/myapp
  
  web:
    env_file:
      - ../.env
    build:
      context: ..
      dockerfile: .devcontainer/Dockerfile
    volumes:
      - ..:/workspaces/rails_app:cached
    command: sleep infinity
    ports:
      - 3306:3306
    depends_on:
      - db

volumes:
  db-store:
```

#### Dockerfile作成
```dockerfile
FROM mcr.microsoft.com/devcontainers/ruby:latest

RUN apt update && \
    apt-get install --no-install-recommends -y \
    build-essential \
    default-libmysqlclient-dev \
    mariadb-client \
    libvips \
    pkg-config \
    git

RUN su vscode -c "gem install rails:7.2.1"
RUN su vscode -c "/usr/local/rvm/bin/rvm fix-permissions"

WORKDIR "/workspaces/rails_app"
RUN bundle config set --local path vendor/bundle
```

#### 環境変数ファイル作成
**.env**
```
POSTGRES_PASSWORD=root
DEV_DB_NAME=devlopment
TEST_DB_NAME=test
PROD_DB_NAME=production
DB_HOST=db
BINDING=0.0.0.0
```

**.env.sample**
```
POSTGRES_PASSWORD=root-root
DEV_DB_NAME=DevDB
TEST_DB_NAME=TestDB
PROD_DB_NAME=ProdDB
DB_HOST=db
BINDING=0.0.0.0
```

### 4. コンテナ起動
- `shift + ⌘ + P`でコマンドパレット
- `Reopen in Container`を選択

コンテナ立ち上げ後、RailsとRubyが利用可能：
```bash
rails -v  # Rails 7.2.1
ruby -v   # ruby 3.3.4
```

### 5. Railsプロジェクト作成
```bash
rails new . --force --no-deps --database=postgresql
```

### 6. database.yml設定
PostgreSQL用にdatabase.ymlを環境変数ベースで設定：

```yaml
default: &default
  adapter: postgresql
  encoding: unicode
  pool: <%= ENV.fetch("RAILS_MAX_THREADS") { 5 } %>
  username: postgres
  password: <%= ENV["POSTGRES_PASSWORD"] %>
  host: <%= ENV["DB_HOST"] %>

development:
  <<: *default
  database: <%= ENV["DEV_DB_NAME"] %>

test:
  <<: *default
  database: <%= ENV["TEST_DB_NAME"] %>

production:
  <<: *default
  database: <%= ENV["PROD_DB_NAME"] %>
```

### 7. .gitignore更新
```
/.bundle
/vendor/bundle
!/.env.sample
```

### 8. セットアップ完了
```bash
bin/setup
rails s
```

`localhost:3000`で接続確認。

## 追記: psqlコマンド利用
DevContainer内でpsqlコマンドを使用する場合：
```bash
sudo apt install postgresql postgresql-contrib
```

## タグ
#DevContainer #Rails #PostgreSQL #VSCode #Docker #Ruby #環境構築
