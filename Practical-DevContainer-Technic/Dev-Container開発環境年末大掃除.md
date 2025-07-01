# Dev Containerで開発環境の年末大掃除を行う

> **原文**: [Dev Containerで開発環境の年末大掃除を行う](https://zenn.dev/i_do/articles/c72fc2867d8c58)

## 概要

開発環境の「年末大掃除」として、ローカル環境を汚さないDev Containerの理解を深め、Python開発環境を1から構築する実践記録。Docker基礎からセキュリティ考慮まで含む学習プロセス。

## Dev Containerとは

### 基本概念
- **定義**: VS Codeの拡張機能の1つ
- **機能**: 開発環境を丸ごとコンテナ化
- **メリット**: ローカル環境を汚さずに開発が可能

### 個人的に魅力的なポイント
1. **ローカル環境保護**: local環境を汚さない
2. **再現性**: GitHub等に上げてPC変更時の設定不要
3. **ポータビリティ**: チーム間での環境共有

## Dev Containerの動作仕組み

### アーキテクチャ
```
ホストマシン
├── VS Code（ローカル）
├── Docker Engine
└── Dev Container
    ├── 開発環境（言語・ツール）
    ├── VS Code Server
    └── マウントされたソースコード
```

### データの流れ
1. **ビルド**: Dockerfileからコンテナ作成
2. **マウント**: ソースコードをボリュームとしてマウント
3. **接続**: VS CodeからコンテナのVS Code Serverに接続
4. **開発**: コンテナ内でコード実行・デバッグ

## 必要なファイル構成

### ディレクトリ構造
```
project-root/
├── .devcontainer/
│   ├── devcontainer.json（必須）
│   ├── Dockerfile（任意）
│   └── docker-compose.yml（任意）
└── src/
    └── main.py
```

## Python環境の実装例

### Dockerfile
```dockerfile
FROM debian:trixie-20231120-slim

ARG USERNAME=vscode
ARG GROUPNAME=developer
ARG UID=1000
ARG GID=1000

# Python環境のセットアップ
RUN apt-get update \
    && apt-get install python3 -y \
    && apt-get install python3-pip -y

# 開発用ユーザーの作成
RUN groupadd -g $GID $GROUPNAME && \
    useradd -m -s /bin/bash -u $UID -g $GID $USERNAME

USER $USERNAME
WORKDIR /home/vscode
```

### docker-compose.yml
```yaml
version: "3.8"
services:
  devcontainer:
    build:
      context: ..
      dockerfile: .devcontainer/Dockerfile
    volumes:
      - ..:/home/vscode:cached
    command: /bin/sh -c "while sleep 1000; do :; done"
    user: vscode

volumes:
  project-name:
```

### devcontainer.json
```json
{
  "name": "proj-{project-name}",
  "service": "devcontainer",
  "dockerComposeFile": "docker-compose.yml",
  "workspaceFolder": "/home/vscode",
  "customizations": {
    "vscode": {
      "extensions": [
        "GitHub.vscode-pull-request-github",
        "ms-python.python"
      ]
    }
  },
  "remoteUser": "vscode",
  "features": {
    "ghcr.io/devcontainers/features/github-cli:1": {}
  }
}
```

## 重要な設定の解説

### 1. コンテナの永続化
```yaml
command: /bin/sh -c "while sleep 1000; do :; done"
```
- **目的**: コンテナを永続的に稼働させる
- **必要性**: これがないと「Container is not running」エラーが発生

### 2. VS Code拡張機能の自動インストール
```json
"customizations": {
  "vscode": {
    "extensions": [
      "GitHub.vscode-pull-request-github",
      "ms-python.python"
    ]
  }
}
```
- **効果**: devcontainer起動時に指定拡張機能が自動インストール
- **カスタマイズ**: プロジェクトに必要な拡張機能を事前定義

### 3. Dev Container Features
```json
"features": {
  "ghcr.io/devcontainers/features/github-cli:1": {}
}
```
- **機能**: gitコマンドを使用可能にする
- **注意**: Pythonの公式featureは動作しない場合があり（OS問題）

## セキュリティ・権限管理の重要性

### 実行ユーザーの問題

#### デフォルト（root）の問題
```dockerfile
# デフォルトではrootユーザー
USER root  # これは危険
```

**問題点**:
1. **ファイル所有権**: コンテナ内で作成したファイルの所有者がroot
2. **セキュリティリスク**: コンテナが乗っ取られた場合のホスト攻撃リスク
3. **権限問題**: ホスト側でroot権限なしにファイル操作不可

#### 解決策：専用ユーザー作成
```dockerfile
ARG USERNAME=vscode
ARG GROUPNAME=developer
ARG UID=1000
ARG GID=1000

RUN groupadd -g $GID $GROUPNAME && \
    useradd -m -s /bin/bash -u $UID -g $GID $USERNAME

USER $USERNAME
```

**メリット**:
- **適切な権限**: ファイル所有権が適切に設定
- **セキュリティ向上**: 最小権限の原則
- **ホスト操作**: 権限問題なしにファイル操作可能

## 環境固有の問題と解決

### Debianイメージの選択
```dockerfile
# 問題のあったバージョン
FROM debian:trixie-20231120-slim

# 解決したバージョン
FROM debian:bullseye-slim
```

**発生した問題**:
- pipコマンドが使用できない
- イメージ固有の問題と判明

**解決方法**:
- より安定したbullseyeイメージに変更
- [GitHub Community Discussion参考](https://github.com/orgs/community/discussions/61327)

## 苦労した点・学んだこと

### 権限設定の理解が重要
```bash
# rootユーザーでファイル作成された場合
-rw-r--r-- 1 root root   100 Dec 10 file.txt

# 一般ユーザーでファイル作成された場合  
-rw-r--r-- 1 vscode vscode 100 Dec 10 file.txt
```

### マウントとの関係
- **ボリュームマウント**: `..:/home/vscode:cached`
- **権限の継承**: コンテナ内のユーザー設定がファイルシステムに影響
- **ホスト操作**: 適切なUID/GIDでホスト側からも操作可能

## 起動・運用手順

### 1. Dev Container起動
```
VS Code Command Palette
> Dev Containers: Reopen in Container
```

### 2. 動作確認
```bash
# Python動作確認
python3 --version

# pip動作確認
pip3 --version

# 拡張機能確認
# VS Codeで拡張機能タブを確認
```

### 3. 開発開始
- 通常のVS Code操作
- コンテナ内でコード実行
- デバッグ機能利用可能

## 年末大掃除の効果

### Before（学生時代）
- **ローカル環境**: 複数言語・ツールでごちゃごちゃ
- **環境破綻**: 依存関係の競合でトラブル多発
- **再現困難**: PC変更時に環境再構築が困難

### After（Dev Container）
- **ローカル環境**: クリーンな状態を維持
- **隔離環境**: プロジェクトごとに独立した環境
- **再現性**: どこでも同じ開発環境

## 今後の展望

### 発展的な活用
- **チーム開発**: 環境統一による効率化
- **CI/CD連携**: 本番環境との整合性向上
- **実験環境**: 新技術の試行が安全

### 継続学習
- **より複雑な構成**: マルチコンテナ環境
- **パフォーマンス最適化**: ビルド時間短縮
- **セキュリティ強化**: より安全な設定

## まとめ

Dev Containerによる開発環境構築で得られたもの：

1. **環境の分離**: プロジェクトごとの独立した環境
2. **再現性**: チーム・デバイス間での環境統一
3. **学習機会**: Docker・セキュリティの理解深化
4. **開発効率**: 環境構築時間の大幅短縮

特に「ローカル環境を汚さない」メリットは、複数プロジェクトを扱う開発者には大きな価値。来年以降もこの方式で開発継続予定。