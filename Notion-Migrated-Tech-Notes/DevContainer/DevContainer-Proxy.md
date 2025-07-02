# Proxy環境下でDevContainerを扱う上での注意点について

## メタデータ
- **作成日時**: 2024-07-25T05:40:00.000Z
- **最終更新**: 2024-07-25T11:22:00.000Z
- **タグ**: #DevContainer #Proxy #Network #Enterprise
- **移植元**: Notion
- **移植日**: 2025-07-02

## 概要

企業や組織のProxy環境下でDevContainerを使用する際の設定方法と注意点についてまとめたページです。

## Proxy環境での主な課題

### 1. ネットワーク接続の問題
- パッケージのダウンロードができない
- Gitクローンが失敗する
- Docker Hub等のレジストリにアクセスできない

### 2. SSL証明書の問題
- 自己署名証明書の検証エラー
- 企業内CA証明書の認識されない

### 3. 認証の問題
- Proxy認証が必要な場合の設定
- ユーザー名・パスワードの管理

## DevContainer設定でのProxy対応

### devcontainer.json の設定
```json
{
  "name": "Proxy Environment",
  "image": "mcr.microsoft.com/devcontainers/base:ubuntu",
  "containerEnv": {
    "HTTP_PROXY": "http://proxy.company.com:8080",
    "HTTPS_PROXY": "http://proxy.company.com:8080",
    "NO_PROXY": "localhost,127.0.0.1,.company.com",
    "http_proxy": "http://proxy.company.com:8080",
    "https_proxy": "http://proxy.company.com:8080",
    "no_proxy": "localhost,127.0.0.1,.company.com"
  },
  "build": {
    "args": {
      "HTTP_PROXY": "http://proxy.company.com:8080",
      "HTTPS_PROXY": "http://proxy.company.com:8080",
      "NO_PROXY": "localhost,127.0.0.1,.company.com"
    }
  }
}
```

### Dockerfile でのProxy設定
```dockerfile
FROM mcr.microsoft.com/devcontainers/base:ubuntu

# Proxy設定を環境変数として設定
ARG HTTP_PROXY
ARG HTTPS_PROXY
ARG NO_PROXY

ENV http_proxy=$HTTP_PROXY
ENV https_proxy=$HTTPS_PROXY
ENV no_proxy=$NO_PROXY

# apt プロキシ設定
RUN echo "Acquire::http::Proxy \"$HTTP_PROXY\";" > /etc/apt/apt.conf.d/proxy.conf && \
    echo "Acquire::https::Proxy \"$HTTPS_PROXY\";" >> /etc/apt/apt.conf.d/proxy.conf

# パッケージ更新
RUN apt-get update && apt-get install -y \
    curl \
    wget \
    git \
    && rm -rf /var/lib/apt/lists/*
```

## 言語・ツール別のProxy設定

### Node.js / npm
```bash
# npmのProxy設定
npm config set proxy http://proxy.company.com:8080
npm config set https-proxy http://proxy.company.com:8080

# 信頼できないSSL証明書を無視（開発環境のみ）
npm config set strict-ssl false
```

```json
// .npmrc ファイル
proxy=http://proxy.company.com:8080
https-proxy=http://proxy.company.com:8080
strict-ssl=false
registry=https://registry.npmjs.org/
```

### Python / pip
```bash
# pipのProxy設定
pip config set global.proxy http://proxy.company.com:8080
```

```ini
# ~/.pip/pip.conf
[global]
proxy = http://proxy.company.com:8080
trusted-host = pypi.org
               pypi.python.org
               files.pythonhosted.org
```

### Git
```bash
# GitのProxy設定
git config --global http.proxy http://proxy.company.com:8080
git config --global https.proxy http://proxy.company.com:8080

# SSL証明書の検証を無効化（開発環境のみ）
git config --global http.sslVerify false
```

### Rust / Cargo
```toml
# ~/.cargo/config.toml
[http]
proxy = "proxy.company.com:8080"

[net]
git-fetch-with-cli = true
```

## Docker daemon のProxy設定

### Docker Desktop (Windows/Mac)
```json
// ~/.docker/config.json
{
  "proxies": {
    "default": {
      "httpProxy": "http://proxy.company.com:8080",
      "httpsProxy": "http://proxy.company.com:8080",
      "noProxy": "localhost,127.0.0.1,.company.com"
    }
  }
}
```

### Linux Docker daemon
```json
// /etc/docker/daemon.json
{
  "proxies": {
    "http-proxy": "http://proxy.company.com:8080",
    "https-proxy": "http://proxy.company.com:8080",
    "no-proxy": "localhost,127.0.0.1,.company.com"
  }
}
```

## 認証付きProxyの設定

### ユーザー名・パスワード認証
```bash
# 環境変数での設定
export HTTP_PROXY="http://username:password@proxy.company.com:8080"
export HTTPS_PROXY="http://username:password@proxy.company.com:8080"
```

### セキュアな認証情報管理
```json
// devcontainer.json
{
  "containerEnv": {
    "HTTP_PROXY": "${localEnv:HTTP_PROXY}",
    "HTTPS_PROXY": "${localEnv:HTTPS_PROXY}"
  }
}
```

```bash
# ホスト側の .env ファイル
HTTP_PROXY=http://username:password@proxy.company.com:8080
HTTPS_PROXY=http://username:password@proxy.company.com:8080
```

## SSL証明書の問題への対処

### 企業CA証明書の追加
```dockerfile
# 企業CA証明書をコンテナに追加
COPY certificates/company-ca.crt /usr/local/share/ca-certificates/
RUN update-ca-certificates
```

### Node.js での証明書設定
```bash
# 証明書チェックを無効化（開発環境のみ）
export NODE_TLS_REJECT_UNAUTHORIZED=0
```

## トラブルシューティング

### よくある問題と解決策

#### 1. DNS解決の問題
```json
// devcontainer.json
{
  "runArgs": [
    "--dns=8.8.8.8",
    "--dns=8.8.4.4"
  ]
}
```

#### 2. タイムアウトエラー
```bash
# curlのタイムアウト設定
export CURL_CA_BUNDLE=""
curl --connect-timeout 30 --max-time 300 -O <URL>
```

#### 3. パッケージインストールの失敗
```dockerfile
# リトライ機能付きのインストール
RUN for i in 1 2 3; do \
    apt-get update && apt-get install -y <packages> && break || sleep 15; \
    done
```

## 動作確認コマンド

### 接続テスト
```bash
# Proxy経由での接続確認
curl -I --proxy http://proxy.company.com:8080 https://www.google.com

# 環境変数の確認
env | grep -i proxy

# DNS確認
nslookup google.com
```

### パッケージマネージャーテスト
```bash
# npm
npm config list

# pip
pip config list

# apt
apt-get update
```

## ベストプラクティス

### 1. 設定の外部化
- 環境固有の設定はホスト側で管理
- 機密情報はコンテナイメージに含めない

### 2. 段階的デバッグ
- 基本的な接続から確認
- 各ツールごとに個別にテスト

### 3. 文書化
- 環境特有の設定は README に記載
- チーム内での設定共有

## 関連リンク
- [[DevContainer-CLI]] - DevContainer CLIの活用
- [[Bun-DevContainer]] - Bun環境での設定
- [[Prisma-DevContainer]] - Prisma環境での設定

## ノート
- このページはNotionから移植されました
- Proxy設定は環境に依存するため、IT部門と連携することを推奨
- セキュリティ設定を緩める場合は開発環境に限定し、本番環境では適用しない
- 企業ポリシーに従って適切な設定を行ってください
