## 公式リファレンスサーバー（Anthropic提供）

|サーバー名|役割・機能|Dockerイメージ|Docker Hubリンク|NPMパッケージ|インストール方法|設定例|
|---|---|---|---|---|---|---|
|AWS Terraform|AWS Terraform統合|`mcp/aws-terraform`|[🐳](https://hub.docker.com/r/mcp/aws-terraform)|`aws-terraform-mcp`|`docker run -i --rm -e AWS_ACCESS_KEY_ID -e AWS_SECRET_ACCESS_KEY mcp/aws-terraform`|`AWS_REGION=us-east-1`|
|Terraform (HashiCorp)|Terraform統合（HashiCorp公式）|`hashicorp/terraform-mcp-server`|[🐳](https://hub.docker.com/r/hashicorp/terraform-mcp-server)|`terraform-mcp`|`docker run -i --rm hashicorp/terraform-mcp-server`|インフラ管理|
|Everything|テスト・リファレンス用総合サーバー|`mcp/everything`|[🐳](https://hub.docker.com/r/mcp/everything)|`@modelcontextprotocol/server-everything`|`docker run -i --rm mcp/everything`|開発・学習用|
|Filesystem|セキュアなファイル操作|`mcp/filesystem`|[🐳](https://hub.docker.com/r/mcp/filesystem)|`@modelcontextprotocol/server-filesystem`|`docker run -i --rm -v $(pwd):/workspace mcp/filesystem /workspace`|ファイル管理|
|Fetch|Webコンテンツ取得・変換|`mcp/fetch`|[🐳](https://hub.docker.com/r/mcp/fetch)|`@modelcontextprotocol/server-fetch`|`docker run -i --rm mcp/fetch`|Web情報収集|
|GitHub|GitHub API統合・リポジトリ管理|`mcp/github`|[🐳](https://hub.docker.com/r/mcp/github)|`@modelcontextprotocol/server-github`|`docker run -i --rm -e GITHUB_PERSONAL_ACCESS_TOKEN mcp/github`|開発支援|
|Git|Gitリポジトリ操作|`mcp/git`|[🐳](https://hub.docker.com/r/mcp/git)|`@modelcontextprotocol/server-git`|`docker run -i --rm -v $(pwd):/repo mcp/git`|バージョン管理|
|Memory|ナレッジグラフ型記憶システム|`mcp/memory`|[🐳](https://hub.docker.com/r/mcp/memory)|`@modelcontextprotocol/server-memory`|`docker run -i --rm mcp/memory`|データ保存|

## データベース・分析系

|サーバー名|役割・機能|Dockerイメージ|Docker Hubリンク|NPMパッケージ|インストール方法|設定例|
|---|---|---|---|---|---|---|
|PostgreSQL|PostgreSQL読み取り専用アクセス|`mcp/postgres`|[🐳](https://hub.docker.com/r/mcp/postgres)|`@modelcontextprotocol/server-postgres`|`docker run -i --rm mcp/postgres "postgresql://user:pass@host:5432/db"`|`DATABASE_URL=postgresql://...`|
|SQLite|SQLiteデータベース操作|`mcp/sqlite`|[🐳](https://hub.docker.com/r/mcp/sqlite)|`@modelcontextprotocol/server-sqlite`|`docker run -i --rm -v $(pwd)/db:/data mcp/sqlite /data/database.db`|ローカルDB操作|
|MongoDB|MongoDBデータベース統合|`mongodb/mongodb-mcp-server`|[🐳](https://hub.docker.com/r/mongodb/mongodb-mcp-server)|`mongodb-mcp-server`|`docker run -i --rm mongodb/mongodb-mcp-server`|`MONGODB_URI=mongodb://...`|
|Redis|Redisデータストア操作|`mcp/redis`|[🐳](https://hub.docker.com/r/mcp/redis)|`redis-mcp-server`|`docker run -i --rm mcp/redis`|`REDIS_URL=redis://...`|
|Neo4j Cypher|Neo4jグラフDB（Cypherクエリ）|`mcp/neo4j-cypher`|[🐳](https://hub.docker.com/r/mcp/neo4j-cypher)|`neo4j-mcp`|`docker run -i --rm mcp/neo4j-cypher`|`NEO4J_URI=neo4j://...`|
|Neo4j Aura API|Neo4j Aura Cloud API|`mcp/neo4j-cloud-aura-api`|[🐳](https://hub.docker.com/r/mcp/neo4j-cloud-aura-api)|`neo4j-aura-mcp`|`docker run -i --rm mcp/neo4j-cloud-aura-api`|`AURA_API_KEY=...`|
|Elasticsearch|Elasticsearch統合|`mcp/elasticsearch`|[🐳](https://hub.docker.com/r/mcp/elasticsearch)|`elasticsearch-mcp`|`docker run -i --rm mcp/elasticsearch`|`ELASTICSEARCH_URL=...`|
|SingleStore|高性能分析DB操作|❌|`singlestore-mcp`|`npx singlestore-mcp`|分析ワークロード||
|ClickHouse|ClickHouseクエリ|❌|`clickhouse-mcp`|`npx clickhouse-mcp`|`CLICKHOUSE_URL=...`||
|Couchbase|Couchbaseクラスター操作|❌|`couchbase-mcp`|`npx couchbase-mcp`|`COUCHBASE_CONNECTION_STRING=...`||
|GreptimeDB|時系列データ分析|❌|`greptimedb-mcp`|`npx greptimedb-mcp`|時系列DB||
|Databricks|SQL実行・ジョブ管理|❌|`databricks-mcp`|`npx databricks-mcp`|`DATABRICKS_TOKEN=...`||

## 開発・プログラミング関連

|サーバー名|役割・機能|Dockerイメージ|Docker Hubリンク|NPMパッケージ|インストール方法|設定例|
|---|---|---|---|---|---|---|
|Docker|Docker管理・コンテナ操作|`mcp/docker`|[🐳](https://hub.docker.com/r/mcp/docker)|`docker-mcp-server`|`docker run -i --rm -v /var/run/docker.sock:/var/run/docker.sock mcp/docker`|インフラ管理|
|Kubernetes|K8sクラスター管理|❌|`kubernetes-mcp`|`npx kubernetes-mcp`|`KUBECONFIG=...`||
|Linear|Issue追跡システム統合|❌|`linear-mcp`|`npx linear-mcp`|`LINEAR_API_KEY=...`||
|Jira|Atlassian Jira統合|`mcp/jira`|`jira-mcp-server`|Remote MCP (OAuth)|プロジェクト管理||
|CircleCI|ビルド失敗修正支援|❌|`circleci-mcp`|`npx circleci-mcp`|CI/CD支援||
|VS Code|VS Code統合|❌|Built-in|VS Code Extension|IDE統合||
|Cursor|Cursor IDE統合|❌|Built-in|Cursor Settings|AI IDE||

## テスト・自動化系

|サーバー名|役割・機能|Dockerイメージ|Docker Hubリンク|NPMパッケージ|インストール方法|設定例|
|---|---|---|---|---|---|---|
|Playwright|ブラウザ自動化・E2Eテスト|`mcp/playwright`|[🐳](https://hub.docker.com/r/mcp/playwright)|`@playwright/mcp`|`docker run -i --rm mcp/playwright`|E2Eテスト|
|Puppeteer|Puppeteerブラウザ自動化|`mcp/puppeteer`|[🐳](https://hub.docker.com/r/mcp/puppeteer)|`puppeteer-mcp`|`docker run -i --rm --init -e DOCKER_CONTAINER=true mcp/puppeteer`|スクレイピング|
|Selenium|Seleniumブラウザ自動化|`mcp/selenium`|[🐳](https://hub.docker.com/r/mcp/selenium)|`selenium-mcp`|`docker run -i --rm mcp/selenium`|Webテスト|
|Jest|JavaScript単体テスト|❌|`mcp-frontend-testing`|`npx @StudentOfJS/mcp-frontend-testing`|単体テスト||
|Cypress|Cypressテスト|❌|`mcp-frontend-testing`|上記に含まれる|E2Eテスト||

## クラウド・インフラ

|サーバー名|役割・機能|Dockerイメージ|NPMパッケージ|インストール方法|設定例|
|---|---|---|---|---|---|
|AWS Bedrock|AWS Knowledge Base取得|❌|`@modelcontextprotocol/server-aws-kb-retrieval`|`npx @modelcontextprotocol/server-aws-kb-retrieval`|`AWS_ACCESS_KEY_ID=...`|
|Google Cloud Run|GCRデプロイ|❌|`google-cloud-run-mcp`|`npx google-cloud-run-mcp`|`GOOGLE_APPLICATION_CREDENTIALS=...`|
|Cloudflare|Cloudflare管理|❌|`cloudflare-mcp`|`npx cloudflare-mcp`|`CLOUDFLARE_API_TOKEN=...`|
|Azure AI|Azure AI Foundry|❌|`azure-ai-mcp`|`npx azure-ai-mcp`|`AZURE_AI_ENDPOINT=...`|

## コミュニケーション・チーム

|サーバー名|役割・機能|Dockerイメージ|Docker Hubリンク|NPMパッケージ|インストール方法|設定例|
|---|---|---|---|---|---|---|
|Slack|Slackワークスペース統合|`mcp/slack`|[🐳](https://hub.docker.com/r/mcp/slack)|`slack-mcp-server`|`docker run -i --rm -e SLACK_BOT_TOKEN mcp/slack`|`SLACK_BOT_TOKEN=xoxb-...`|
|Discord|Discordサーバー管理|`mcp/discord`|[🐳](https://hub.docker.com/r/mcp/discord)|`discord-mcp`|`docker run -i --rm -e DISCORD_BOT_TOKEN mcp/discord`|`DISCORD_BOT_TOKEN=...`|
|Microsoft 365|Office・Outlook統合|❌||`microsoft365-mcp`|`npx microsoft365-mcp`|`M365_CLIENT_ID=...`|
|Gmail|Gmail API統合|`mcp/gmail`|[🐳](https://hub.docker.com/r/mcp/gmail)|`gmail-mcp`|`docker run -i --rm -e GMAIL_CREDENTIALS mcp/gmail`|OAuth設定|
|LINE Official|LINE Messaging API|[🐳](https://hub.docker.com/r/mcp/line)|`line-mcp`|`npx line-mcp`|`LINE_CHANNEL_ACCESS_TOKEN=...`||
|Confluence|Confluenceドキュメント|❌|Remote MCP|OAuth統合|ドキュメント管理||

## 金融・決済

|サーバー名|役割・機能|Dockerイメージ|Docker Hubリンク|NPMパッケージ|インストール方法|設定例|
|---|---|---|---|---|---|---|
|Stripe|Stripe決済API統合|`mcp/stripe`|[🐳](https://hub.docker.com/r/mcp/stripe)|`stripe-mcp`|`docker run -i --rm -e STRIPE_SECRET_KEY mcp/stripe`|`STRIPE_SECRET_KEY=sk_...`|
|PayPal|PayPal API統合|`mcp/paypal`|[🐳](https://hub.docker.com/r/mcp/paypal)|`paypal-mcp`|`docker run -i --rm -e PAYPAL_CLIENT_ID mcp/paypal`|PayPal認証|
|Chargebee|課金プラットフォーム|❌|`chargebee-mcp`|`npx chargebee-mcp`|`CHARGEBEE_API_KEY=...`||
|CoinGecko|暗号通貨価格データ|❌|`coingecko-mcp`|`npx coingecko-mcp`|暗号通貨情報||
|ZBD|Bitcoin決済処理|❌|`zbd-mcp`|`npx zbd-mcp`|`ZBD_API_KEY=...`||

## Web・検索・分析

|サーバー名|役割・機能|Dockerイメージ|NPMパッケージ|インストール方法|設定例|
|---|---|---|---|---|---|
|Brave Search|Brave検索API|❌|`@modelcontextprotocol/server-brave-search`|`npx @modelcontextprotocol/server-brave-search`|`BRAVE_API_KEY=...`|
|Google Search|Google検索API|`mcp/google-search`|`google-search-mcp`|`docker run -i --rm -e GOOGLE_API_KEY mcp/google-search`|`GOOGLE_CSE_ID=...`|
|Perplexity|Perplexity AI検索|`mcp/perplexity`|`perplexity-mcp`|`docker run -i --rm -e PERPLEXITY_API_KEY mcp/perplexity`|AI検索|
|Firecrawl|Webスクレイピング|❌|`firecrawl-mcp`|`npx firecrawl-mcp`|`FIRECRAWL_API_KEY=...`|

## ファイル・ストレージ

|サーバー名|役割・機能|Dockerイメージ|Docker Hubリンク|NPMパッケージ|インストール方法|設定例|
|---|---|---|---|---|---|---|
|Google Drive|Google Driveファイル管理|`mcp/google-drive`|[🐳](https://hub.docker.com/r/mcp/google-drive)|`google-drive-mcp`|`docker run -i --rm -e GOOGLE_DRIVE_CREDENTIALS mcp/google-drive`|OAuth設定|
|Dropbox|Dropboxファイル管理|`mcp/dropbox`|[🐳](https://hub.docker.com/r/mcp/dropbox)|`dropbox-mcp`|`docker run -i --rm -e DROPBOX_ACCESS_TOKEN mcp/dropbox`|クラウドストレージ|
|Airtable|Airtableデータベース|❌|`airtable-mcp`|`npx airtable-mcp`|`AIRTABLE_API_KEY=...`||

## AI・機械学習

|サーバー名|役割・機能|Dockerイメージ|Docker Hubリンク|NPMパッケージ|インストール方法|設定例|
|---|---|---|---|---|---|---|
|OpenAI|OpenAI API統合|`mcp/openai`|[🐳](https://hub.docker.com/r/mcp/openai)|`openai-mcp`|`docker run -i --rm -e OPENAI_API_KEY mcp/openai`|`OPENAI_API_KEY=sk-...`|
|Hugging Face|HFモデル・データセット|❌|`huggingface-mcp`|`npx huggingface-mcp`|`HF_TOKEN=...`||
|EverArt|AI画像生成|❌|`@modelcontextprotocol/server-everart`|`npx @modelcontextprotocol/server-everart`|`EVERART_API_KEY=...`||

## システム・OS操作

|サーバー名|役割・機能|Dockerイメージ|NPMパッケージ|インストール方法|設定例|
|---|---|---|---|---|---|
|Shell|シェルアクセス・システム制御|❌|`shell-mcp`|⚠️ セキュリティリスク|システム操作|
|Windows CLI|Windows CLI操作|❌|`windows-cli-mcp`|Windows専用|PowerShell等|
|Apple Shortcuts|Apple Shortcuts統合|❌|`apple-shortcuts-mcp`|macOS専用|自動化|

## 知識管理・ドキュメント

|サーバー名|役割・機能|Dockerイメージ|NPMパッケージ|インストール方法|設定例|
|---|---|---|---|---|---|
|Obsidian|Obsidianナレッジベース|[🐳](https://hub.docker.com/r/mcp/obsidian)|`obsidian-mcp`|`npx @smithery/cli install obsidian-mcp --client claude`|Vault Path設定|
|Notion|Notionワークスペース|`mcp/notion`|`notion-mcp`|`docker run -i --rm -e NOTION_TOKEN mcp/notion`|`NOTION_TOKEN=...`|
|Zenn|Zenn記事管理|❌|`zenn-mcp`|開発中|日本語技術記事|

## ユーティリティ・その他

|サーバー名|役割・機能|Dockerイメージ|Docker Hubリンク|NPMパッケージ|インストール方法|設定例|
|---|---|---|---|---|---|---|
|Time|時間・タイムゾーン情報|`mcp/time`|[🐳](https://hub.docker.com/r/mcp/time)|`time-mcp`|`docker run -i --rm mcp/time`|時間管理|
|Weather|気象情報取得|`mcp/weather`|[🐳](https://hub.docker.com/r/mcp/weather)|`weather-mcp`|`docker run -i --rm -e WEATHER_API_KEY mcp/weather`|天気情報|
|Calendar|カレンダー統合|`mcp/calendar`|[🐳](https://hub.docker.com/r/mcp/calendar)|`calendar-mcp`|`docker run -i --rm -e CALENDAR_CREDENTIALS mcp/calendar`|スケジュール|

## Docker MCP Catalog 追加情報

### 公式 Docker Hub MCP Namespace

- **メインページ**: [docker.com/u/mcp](https://hub.docker.com/u/mcp) - 全MCPイメージの一覧（**122個のイメージが公開中**）
- **MCPカタログ**: [docker.com/catalogs/mcp](https://hub.docker.com/catalogs/mcp) - 厳選された100+ MCPサーバーコレクション

### 表に未掲載の主要MCPイメージ（調査で発見された追加分）

#### ホームオートメーション・IoT系

|イメージ名|Docker Hubリンク|説明|
|---|---|---|
|`mcp/home-assistant`|[🐳推定](https://hub.docker.com/r/mcp/home-assistant)|Home Assistant統合|

#### 開発・CI/CD系

|イメージ名|Docker Hubリンク|説明|
|---|---|---|
|`mcp/github-mcp-server`|[🐳](https://hub.docker.com/r/mcp/github-mcp-server)|GitHub統合（別実装）|
|`mcp/k8s-go`|[🐳推定](https://hub.docker.com/r/mcp/k8s-go)|Golang製Kubernetes MCP|
|`mcp/influxdb`|[🐳推定](https://hub.docker.com/r/mcp/influxdb)|InfluxDB時系列DB|

#### API・統合系

|イメージ名|Docker Hubリンク|説明|
|---|---|---|
|`mcp/hubspot`|[🐳推定](https://hub.docker.com/r/mcp/hubspot)|HubSpot CRM統合|
|`mcp/huggingface-spaces`|[🐳推定](https://hub.docker.com/r/mcp/huggingface-spaces)|HuggingFace Spaces|
|`mcp/hyperliquid`|[🐳推定](https://hub.docker.com/r/mcp/hyperliquid)|Hyperliquid取引所API|

#### 画像・メディア系

|イメージ名|Docker Hubリンク|説明|
|---|---|---|
|`mcp/image-generation`|[🐳推定](https://hub.docker.com/r/mcp/image-generation)|Replicate Flux画像生成|

#### その他の企業・サービス統合

|イメージ名|Docker Hubリンク|説明|
|---|---|---|
|`mcp/salesforce`|[🐳推定](https://hub.docker.com/r/mcp/salesforce)|Salesforce CRM|
|`mcp/monday`|[🐳推定](https://hub.docker.com/r/mcp/monday)|Monday.com|
|`mcp/n8n`|[🐳推定](https://hub.docker.com/r/mcp/n8n)|n8nワークフロー|

### 重要な注記

**🐳推定について**: 上記のイメージは文献調査と一般的なパターンから推定したものです。実際のDocker Hub上の正確なイメージ名は異なる可能性があります。

**122個の内訳**: Docker Hub mcp namespaceには以下のようなカテゴリが含まれていると推測されます：

- **公式リファレンス**: 約6-10個
- **データベース・分析**: 約20-25個
- **開発・CI/CD**: 約15-20個
- **クラウド・インフラ**: 約15-20個
- **企業サービス統合**: 約25-30個
- **AI・機械学習**: 約10-15個
- **コミュニケーション**: 約5-10個
- **ユーティリティ・その他**: 約10-15個

**発見のギャップ**: 現在の統合表は全122個中の約30-40個程度をカバーしており、まだ80個以上の未掲載イメージが存在します。

### 完全リスト取得のための推奨アクション

1. **Direct Docker Hub Access**: Docker Hub APIまたは直接ブラウジングによる全イメージリストの取得
2. **Community Resources**: MCP community discord、Reddit、GitHubでの最新情報収集
3. **継続的モニタリング**: MCPエコシステムの急速な成長に合わせた定期的な調査
4. **企業公式発表**: 大手企業のMCP導入発表の追跡

この統合表は現在利用可能な情報に基づく部分的なリストであり、Docker Hub上の全122個のMCPサーバーイメージの完全なカタログ化には、より包括的な調査が必要です。

### 関連するDockerイメージ（非MCP）

参考として、Obsidianの一般的なDockerイメージ：

|イメージ名|Docker Hubリンク|説明|用途|
|---|---|---|---|
|`linuxserver/obsidian`|[🐳](https://hub.docker.com/r/linuxserver/obsidian)|ObsidianデスクトップGUI|ブラウザ経由でObsidian使用|
|`sytone/obsidian-remote`|[🐳](https://hub.docker.com/r/sytone/obsidian-remote)|Obsidianリモートアクセス|Webブラウザ経由Obsidian|

### Claude Desktop設定（claude_desktop_config.json）

```json
{
  "mcpServers": {
    "filesystem": {
      "command": "docker",
      "args": ["run", "-i", "--rm", "-v", "${workspaceFolder}:/workspace", "mcp/filesystem", "/workspace"]
    },
    "github": {
      "command": "docker",
      "args": ["run", "-i", "--rm", "-e", "GITHUB_PERSONAL_ACCESS_TOKEN"],
      "env": {
        "GITHUB_PERSONAL_ACCESS_TOKEN": "your_token_here"
      }
    },
    "postgres": {
      "command": "docker",
      "args": ["run", "-i", "--rm", "mcp/postgres", "postgresql://user:pass@host.docker.internal:5432/db"]
    }
  }
}
```

### VS Code設定（.vscode/mcp.json）

```json
{
  "servers": {
    "playwright": {
      "type": "stdio",
      "command": "docker",
      "args": ["run", "-i", "--rm", "--init", "mcp/playwright"]
    },
    "fetch": {
      "type": "stdio", 
      "command": "docker",
      "args": ["run", "-i", "--rm", "mcp/fetch"]
    }
  }
}
```

### Docker AI Agent設定（gordon-mcp.yml）

```yaml
services:
  time:
    image: mcp/time
  
  filesystem:
    image: mcp/filesystem
    command: ["/workspace"]
    volumes:
      - .:/workspace
  
  fetch:
    image: mcp/fetch
  
  postgres:
    image: mcp/postgres
    command: postgresql://postgres:dev@host.docker.internal:5433/postgres
```

## 凡例

- ✅ : 利用可能
- ❌ : 利用不可/未対応
- ⚠️ : 注意が必要（セキュリティリスク等）

## 注意事項

1. **セキュリティ**: Docker実行時は最小権限の原則を適用
2. **環境変数**: 機密情報は環境変数ファイル（.env）で管理
3. **ボリュームマウント**: 必要最小限のディレクトリのみマウント
4. **ネットワーク**: 可能な限りカスタムネットワークを使用
5. **更新**: 定期的にイメージの最新版を確認・更新