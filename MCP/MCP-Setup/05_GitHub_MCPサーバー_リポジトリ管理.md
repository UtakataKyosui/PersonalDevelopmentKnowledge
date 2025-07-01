# MCPのトリセツ #5: GitHub MCPサーバー - AIでリポジトリを管理

## 記事情報
- **著者**: ながたく (takna)
- **URL**: https://zenn.dev/takna/articles/mcp-server-tutorial-05-github
- **シリーズ**: MCPのトリセツ

## 概要
GitHub MCPサーバーを使用してAIとGitHubを直接連携し、Gitコマンドを覚える必要なくリポジトリの操作やコード管理を会話ベースで行う方法を解説。

## GitHub MCPサーバーの特徴
- AIとGitHubの直接連携
- 会話ベースでのリポジトリ操作
- Gitコマンドの知識不要
- 個人アクセストークンのスコープ制限で安全利用

## セットアップ手順

### 1. GitHub個人アクセストークン(PAT)の取得
1. GitHubにログイン
2. 右上のプロフィールアイコン → Settings
3. 左側メニュー → Developer settings
4. Personal access tokens → Tokens (classic)
5. Generate new token (classic)
6. トークン設定:
   - 名前: `My GitHub Token (MCP用)` など
   - 権限: リポジトリ関連の権限（最小限推奨）
   - 有効期限: 30日程度（セキュリティ観点）
7. Generate token → トークンをコピーして安全に保存

**⚠️ セキュリティ注意**: 
- トークンは絶対に公開しない
- GitHubリポジトリや公開設定ファイルに含めない
- 漏洩した場合は即座に無効化して新規発行

### 2. MCPサーバーの設定

#### 方法1: 手動設定

**Claude Desktop**:
`~/Library/Application Support/Claude/claude_desktop_config.json` に追加:

```json
{
  "mcpServers": {
    "github": {
      "command": "npx",
      "args": [
        "-y",
        "@modelcontextprotocol/server-github"
      ],
      "env": {
        "GITHUB_PERSONAL_ACCESS_TOKEN": "ghp_your_token_here"
      }
    }
  }
}
```

**Windsurf**:
`~/.codeium/windsurf/mcp_config.json` に同様の設定を追加

#### 方法2: mcp-installerを使用
```
MCPサーバー @modelcontextprotocol/server-github をインストールして。環境変数 GITHUB_PERSONAL_ACCESS_TOKEN は 'your_token_here' で設定します
```

### 3. 再起動
Claude Desktopを再起動してMCPサーバーを起動

## 使用例（プロンプト）

### 1. リポジトリの基本操作

#### リポジトリ作成
```
Github でリポジトリ "test-repo" を新規作成して

「my-portfolio」という名前の新しいリポジトリを作成して。説明は「個人ポートフォリオサイト」、プライベートリポジトリで、READMEファイルも初期化してください
```

#### README.md作成・編集
```
そのリポジトリに README.md を新規作成して。内容は、Webアプリケーションのテンプレートを説明する基本的な内容でOKです。

README.mdに、プロジェクトの特徴、インストール手順、使用例のセクションを追加してください
```

#### ブランチ管理
```
dev ブランチを作成して

「feature/user-authentication」というブランチを main から作成して
```

#### ファイル追加とコミット
```
index.html を作成し、シンプルなHTMLテンプレートを記述して。その後、コミットして

src/components/Login.js ファイルを作成し、Reactのログインコンポーネントを実装して。コミットメッセージは「ログインフォームコンポーネントの追加」としてください
```

#### プルリクエスト管理
```
「feature/user-authentication」ブランチから「main」ブランチへのプルリクエストを作成してください。タイトルは「ユーザー認証機能の実装」、説明には実装した機能の概要とテスト方法を含めてください

プルリクエスト #5 にコメントを追加して：「CIテストが通過したので、レビューをお願いします」
```

### 2. リポジトリの情報取得と分析

#### リポジトリ検索
```
キーワード "javascript tutorial" で、スターが5000以上のリポジトリを検索して

「react state management」に関する、最近更新された人気のリポジトリを5つ探して、それぞれの特徴を簡潔に説明してください
```

#### コード分析
```
GitHubで「react custom hooks」というキーワードで検索し、よく使われているカスタムフックの実装パターンを3つ見つけて説明してください

リポジトリ「myusername/my-project」内で、セキュリティの脆弱性が疑われるコードパターンを検索して報告してください
```

### 3. 実用的な開発支援

#### コードレビュー
```
私のリポジトリ「myusername/my-project」のコードをレビューして、改善点を指摘してください。特にパフォーマンスとセキュリティの観点から見てください。
```

#### プロジェクト初期化
```
React+TypeScriptのプロジェクトテンプレートを新しいリポジトリ「my-new-app」に作成してください。ESLint、Prettier、Jestのセットアップも含めてください。
```

#### ドキュメント生成
```
リポジトリ「myusername/my-library」のコードから、APIドキュメントを生成して、docs/フォルダに保存してください。各関数の使用例も追加してください。
```

### 4. 高度な活用例

#### セキュリティ監査
```
リポジトリ「myusername/e-commerce-api」のセキュリティ監査を行い、潜在的な脆弱性、ベストプラクティスからの逸脱、改善すべき点を詳細にレポートしてください
```

#### パフォーマンス最適化
```
リポジトリ「myusername/web-app」のフロントエンドコードを分析し、パフォーマンスを最適化するための具体的な提案をしてください。特にレンダリングパフォーマンスとバンドルサイズに注目してください
```

## セキュリティと制限事項
- **最小権限の原則**: 個人アクセストークンには必要最小限の権限のみ付与
- **有効期限の設定**: トークンに適切な有効期限を設定
- **機密リポジトリの扱い**: 重要なプロジェクトでは追加のセキュリティ対策を検討
- **トークンの管理**: 安全に保管し、定期的に更新

## 他のMCPサーバーとの組み合わせ

### Filesystem MCP + GitHub MCP
```
1. /Users/yourname/Projects/my-app/ のコードを分析して、改善点を提案してください。
2. 改善したコードをGitHubリポジトリ「myusername/my-app」にプッシュしてください。
```

### YouTube MCP + GitHub MCP
```
1. このYouTubeチュートリアル（https://www.youtube.com/watch?v=xxxxx）の内容を分析してください。
2. チュートリアルで説明されている機能を実装したコードを新しい GitHubリポジトリに作成してください。
```

## まとめ
GitHub MCPサーバーにより、AIとGitHubの連携が劇的に向上。リポジトリの作成・管理、コードレビュー、Issue管理など、様々な作業を自然言語での会話を通じて実行可能。開発者のワークフロー効率化とGitHub操作の敷居を下げる強力なツール。