# DevContainerで快適開発 - Go + PostgreSQL環境

## 概要
WEBアプリ開発時の環境構築時間を短縮するため、DevContainerを活用したGo + PostgreSQL開発環境の構築方法。DockerfileとDockerComposeを使用したカスタム環境の作成手順を詳しく解説。

## 著者
- **ユーザー**: yDog
- **記事URL**: https://qiita.com/yDog/items/916a4dcf759616f4d65b

## 前提条件
### 環境
- Windows11 Home
- Docker Desktopインストール済み

## DevContainerとは
Dockerのコンテナ技術を利用した、VSCodeで使える開発環境構築ツール。公式テンプレートからの作成に加え、既存のDockerfileやcompose.yamlからも作成可能。

## ファイル構成
```
DevContainer-test/
├── compose.yaml
└── .devcontainer/
    ├── devcontainer.json
    └── Dockerfile
```

## 設定ファイル

### Dockerfile（Go開発用）
```dockerfile
FROM golang:1.22.3-bullseye

RUN useradd -ms /bin/bash dev
USER dev
WORKDIR /app
EXPOSE 8080
COPY . /app
CMD ["go", "mod", "tidy"]
```

### compose.yaml
```yaml
services:
  devcontainer:
    build:
      context: .
      dockerfile: .devcontainer/Dockerfile
    volumes:
      - .:/app
    tty: true
    command: /bin/bash
    ports:
      - "8080:8080"
  
  database:
    image: postgres:13.15-bullseye
    environment:
      POSTGRES_USER: user
      POSTGRES_PASSWORD: password
      POSTGRES_DB: devTest
    ports:
      - "5432:5432"
```

### devcontainer.json
```json
{
  "name": "DevContainer test",
  "dockerComposeFile": "../compose.yaml",
  "service": "devcontainer",
  "workspaceFolder": "/app",
  "customizations": {
    "vscode": {
      "extensions": [
        "golang.go"
      ]
    }
  }
}
```

## devcontainer.jsonプロパティ解説
- **name**: 左下の`><`や検索ボックスに表示される名前
- **dockerComposeFile**: compose.yamlの相対パス
- **service**: 作業対象のservice名
- **workspaceFolder**: コンテナ接続時に自動で開くフォルダ
- **customizations**: サービス固有の設定
  - **extensions**: コンテナ起動時に自動インストールする拡張機能

## 失敗談と解決策

### 1. dockerComposeFileの指定ミス
```json
// ❌ 間違い
"dockerComposeFile": "compose.yaml"

// ✅ 正しい
"dockerComposeFile": "../compose.yaml"
```

### 2. devcontainerサービスの自動停止
compose.yamlに以下を追加してserviceを維持：
```yaml
services:
  devcontainer:
    tty: true
    command: /bin/bash
```

### 3. データベースとの通信問題
個別のDocker Networkではなく、compose.yamlでサービス間通信を設定。

## コンテナのリビルド方法
設定変更後は必ずリビルドが必要：
- `Ctrl + Shift + P`でコマンドパレット表示
- `Dev Containers: Rebuild`を選択

## 感想
- 複雑そうに見えるDevContainerも実際は便利
- WSLでの環境汚染問題を解決
- コンテナベース開発の利点を実感

## タグ
#DevContainer #Go #PostgreSQL #Docker #VSCode #環境構築 #WebAPI
