# モノレポで複数のdevcontainerを使う試み

> **原文**: [モノレポで複数のdevcontainerを使う試み](https://zenn.dev/izm/articles/3651766774c877)

## 概要

バックエンドサーバーとWebフロントエンドが同一GitHubリポジトリに含まれるモノレポ環境で、複数のdevcontainerを使用する開発環境の構築と検証。FastAPI + MySQL + Streamlitの構成での実践例。

## 背景・動機

### モノレポの課題
- **CIの都合**: 複数プロジェクトが同一リポジトリに存在
- **開発環境の統一**: バックエンドとフロントエンドで異なる環境が必要
- **VSCode DevContainer**: 原理的に1リポジトリ1devcontainerが想定

### Devcontainerとは（再確認）
- **基本機能**: DockerやDocker Composeによる実行環境の隔離
- **開発環境も隔離**: 環境差異に苦しまない開発
- **VS Code統合**: ローカルPCから仮想マシン上のVS Code環境を操作

## 複数Devcontainerのアプローチ

### 参考にした手法
参考記事から以下のアプローチを採用：

1. **共通docker-compose.yml**: 重複設定を避けて共通化
2. **複数devcontainer設定**: プロジェクトごとに独立したコンテナ

参考リンク：
- [Dev Containers Part 5: Multiple projects shared container configuration](https://dev.to/graezykev/dev-containers-part-5-multiple-projects-shared-container-configuration-2hoi)
- [multiple dev containerですぐに立ち上げられるモノレポなローカル開発環境を作るぞ！](https://qiita.com/re_2osushi8888/items/e9df614f7b5182ae284d)

## 実装例：FastAPI + MySQL + Streamlit

### プロジェクト構成
```
repository/
├── backend/           # FastAPI
├── frontend/          # Streamlit
├── .devcontainer/
│   ├── backend/
│   │   └── devcontainer.json
│   ├── frontend/
│   │   └── devcontainer.json
│   └── docker-compose.yml
└── README.md
```

### GitHubリポジトリ
実際のサンプル: [PythonFastApiLearn](https://github.com/neon-izm/PythonFastApiLearn)

## セットアップ・使用方法

### 前提条件
1. **Docker Desktop**: 動作する環境
2. **VS Code**: メインエディタ
3. **DevContainer拡張機能**: [Remote Containers](https://marketplace.visualstudio.com/items?itemName=ms-vscode-remote.remote-containers)

### 使用手順
1. **リポジトリクローン**
   ```bash
   git clone https://github.com/neon-izm/PythonFastApiLearn
   cd PythonFastApiLearn
   ```

2. **VS Codeで開く**
   ```bash
   code .
   ```

3. **DevContainer選択**
   - VS Code左下の青いボタンをクリック
   - 「Reopen in Container」を選択
   - Backend（FastAPI）またはFrontend（Streamlit）を選択

4. **環境構築完了**
   - 選択した環境に応じてコンテナが自動構築
   - 適切な開発環境が起動

## 動作確認方法

### Backend（FastAPI）
1. **Swagger UI確認**
   - ブラウザで [http://127.0.0.1:8000/docs](http://127.0.0.1:8000/docs) にアクセス
   - FastAPIのSwagger UIが表示

2. **コード変更テスト**
   ```python
   # backend/routers/users.py
   @router.post("/", response_model=schemas.Token)
   # ↓ エンドポイント名を変更
   @router.post("/test", response_model=schemas.Token)
   ```

3. **即座反映確認**
   - Ctrl+Sで保存
   - ブラウザリロードで変更が即座に反映

### Frontend（Streamlit）
1. **Streamlit UI確認**
   - ブラウザで [http://127.0.0.1:8501/](http://127.0.0.1:8501/) にアクセス
   - StreamlitのWebアプリが表示

2. **コード変更テスト**
   ```python
   # frontend/app.py
   st.title("ログイン/ ユーザー登録")
   # ↓ 表示テキストを変更
   st.title("ログインTest / ユーザー登録")
   ```

3. **即座反映確認**
   - Ctrl+Sで保存
   - ブラウザリロードで変更が即座に反映

## コンテナ管理操作

### コンテナ制御
VS Code左下の青いボタンから以下の操作が可能：

- **Reopen Locally**: ローカル環境に戻る
- **Rebuild Container**: コンテナの再ビルド
- **Reopen in Container**: 別のコンテナに切り替え

### 環境切り替え
- Backend ↔ Frontend間の切り替えが簡単
- 各環境に最適化された開発ツールが自動設定

## 実際に使って感じたメリット

### 良かった点
1. **環境統一**: 他の開発者と同一環境でコード実行
2. **Docker Compose連携**: 既存のDocker環境と相性良好
3. **デプロイ容易性**: 本番環境との整合性
4. **VS Code拡張活用**: 豊富な拡張機能を利用可能
5. **開発イテレーション**: 適度に高速な開発サイクル

### 技術的メリット
- **隔離環境**: プロジェクト間の依存関係衝突回避
- **再現性**: 「私の環境では動く」問題の解決
- **スケーラビリティ**: 新しい環境追加が容易

## 課題・気になった点

### IDE・エディタの制約
- **JetBrains系**: DevContainer統合がVS Codeほどスムーズでない
- **エディタ固定**: VS Code以外の選択肢が限定的

### チーム開発の課題
- **拡張機能の個人差**: お気に入りVS Code拡張のバラつき
- **推奨拡張**: vim拡張などの個人設定をどこまで含めるか
- **合意形成**: チーム内での拡張機能選定の必要性

### 学習コストとトラブルシュート
- **Python環境管理**: venv/poetry/uvなど既存手法との併存
- **新しい作法**: Docker + DevContainerの学習コスト
- **トラブル時の複雑性**:
  - Dockerコンテナのステータス・ログ確認
  - SSH接続でのbash確認
  - ボリュームマウントの確認
  - docker-compose.ymlの理解

### 困難な場面
```bash
# トラブル時に必要な知識・操作
docker ps                          # コンテナ状態確認
docker logs [container_id]         # ログ確認
docker exec -it [container] bash   # コンテナ内調査
docker-compose logs                 # Compose全体ログ
```

## 技術的な実装詳細

### 共通docker-compose.yml
```yaml
version: '3.8'
services:
  backend:
    build: ./backend
    ports:
      - "8000:8000"
    depends_on:
      - mysql
    environment:
      - DATABASE_URL=mysql://user:password@mysql:3306/appdb

  frontend:
    build: ./frontend
    ports:
      - "8501:8501"
    depends_on:
      - backend

  mysql:
    image: mysql:8.0
    environment:
      - MYSQL_ROOT_PASSWORD=rootpassword
      - MYSQL_DATABASE=appdb
    ports:
      - "3306:3306"
```

### Backend DevContainer設定
```json
{
  "name": "FastAPI Backend",
  "dockerComposeFile": "../docker-compose.yml",
  "service": "backend",
  "workspaceFolder": "/workspace",
  "customizations": {
    "vscode": {
      "extensions": [
        "ms-python.python",
        "ms-python.flake8"
      ]
    }
  }
}
```

### Frontend DevContainer設定
```json
{
  "name": "Streamlit Frontend",
  "dockerComposeFile": "../docker-compose.yml",
  "service": "frontend", 
  "workspaceFolder": "/workspace",
  "customizations": {
    "vscode": {
      "extensions": [
        "ms-python.python",
        "ms-toolsai.jupyter"
      ]
    }
  }
}
```

## 今後の展望・発展

### 技術的改善
- **パフォーマンス最適化**: コンテナ起動時間短縮
- **ツール統合**: より多くの開発ツールとの連携
- **デバッグ機能**: より高度なデバッグ環境

### 代替アプローチへの興味
参考記事で紹介されていた方向性：
[モノレポの開発環境でDocker ComposeをやめてTaskfileを導入した話](https://zenn.dev/uzu_tech/articles/e0f0af5945033b)

### 個人的な関心
- **JetBrains統合**: Rider使用者としての今後の動向
- **トラブルシュート改善**: より直感的なエラー対応
- **学習コスト軽減**: より簡単な導入方法

## まとめ

モノレポでの複数DevContainer運用は実用可能であり、以下を実現：

1. **環境分離**: バックエンド・フロントエンドの独立した開発環境
2. **チーム統一**: 開発者間での環境差異解消
3. **効率的開発**: Docker Composeとの連携によるスムーズな開発
4. **再現性**: 「私の環境では動く」問題の根本解決

一方で、学習コストやトラブルシュート時の複雑性といった課題も存在。しかし、適切に設定すれば非常に強力な開発環境を構築可能。

特にモノレポ環境では、複数の技術スタックを扱う際の環境管理が劇的に改善される。今後はより多くのプロジェクトでこの手法が採用されることが期待される。