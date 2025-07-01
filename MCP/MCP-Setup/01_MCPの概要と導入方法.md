# MCPのトリセツ #1: MCPの概要と導入方法

## 記事情報
- **著者**: ながたく (takna)
- **URL**: https://zenn.dev/takna/articles/mcp-server-tutorial-01-install
- **シリーズ**: MCPのトリセツ

## 概要
Claude や Windsurf Cascade などの AI を強化する「MCP（Model Context Protocol）」の導入方法と活用テクニックを初心者向けに解説するシリーズの第1回。

## MCPとは？
**MCP（Model Context Protocol）**: AIにさまざまな機能拡張を安全に追加するための仕組み

### 従来のAIの限界
- インターネット検索はできても、自分のPC内のファイルは読めない
- 画像生成はできても、手元の写真を編集できない

### MCPの利点
- 指定したフォルダのファイルを読み書きできる
- 特定のアプリやWebサービスを操作できる
- その他の拡張機能を追加できる
- AIとPCの間の「通訳」の役割

## MCPサーバーのリソース
- [modelcontextprotocol/servers](https://github.com/modelcontextprotocol/servers) - 公式リポジトリ
- [Glama](https://glama.ai/mcp/servers) - オープンソースMCPサーバー
- [OpenTools.com](https://opentools.com/registry/) - AIツールディレクトリ
- [PulseMCP.com](https://pulsemcp.com/) - MCP専門ディレクトリ
- [Smithery.ai](https://smithery.ai/) - キュレーションサイト

## mcp-installer 導入方法

### 特徴
- **会話で指示**: チャットで簡単にMCPサーバーをインストール
- **設定ファイル自動編集**: JSON設定ファイルを自動更新
- **引数・環境変数対応**: 複雑な設定も会話で指定可能
- **技術知識不要**: コマンドラインやJSON知識が不要

### 使用例
```
MCPサーバー mcp-pandoc をインストールして

MCPサーバー @modelcontextprotocol/server-filesystem をインストールして。引数には['/users/username/desktop']を使用します

MCPサーバー @modelcontextprotocol/server-github をインストールして。環境変数 GITHUB_PERSONAL_ACCESS_TOKEN は後で手動で設定するのでダミー値を入力して
```

### セットアップ手順

#### 1. uvのインストール（macOS）
```bash
brew install uv
uv --version  # 確認
```

#### 2. 設定ファイル編集
**ファイル場所**:
- macOS: `~/Library/Application Support/Claude/claude_desktop_config.json`
- Windows: `C:\Users\ユーザー名\AppData\Roaming\Claude\claude_desktop_config.json`

**設定内容**:
```json
{
  "globalShortcut": "",
  "mcpServers": {
    "mcp-installer": {
      "command": "npx",
      "args": [
        "@anaisbetts/mcp-installer"
      ]
    }
  }
}
```

#### 3. 動作確認
Claude Desktopを再起動後、以下で確認:
```
今、MCP "@anaisbetts/mcp-installer" は使える状態になってる？
```

## シリーズ一覧
1. **MCPの概要と導入方法** (本記事)
2. Filesystem MCP Server: AIでローカルファイルを扱う
3. YouTube MCPサーバー: 動画の内容を取得
4. mcp-pandoc: AIでドキュメント形式を変換
5. GitHub MCPサーバー: AIでリポジトリを管理
6. Figma MCP: デザインとコードを効率的に連携
7. Slack MCPサーバー: チームコミュニケーションを強化
8. Firecrawl MCP: スクレイピングでウェブ情報を取得・分析
9. Markdownify MCP Server: WebページやPDFをMarkdown文書化
10. Raindrop.io MCP Server: ブックマークサービスをAIから使う
11. Fetch MCP Server: ウェブコンテンツを取得・処理
12. Blender MCP Server: 会話でBlenderを操作し3Dモデルを作成
13. Perplexity MCP Server: Perplexityの検索をAIとの会話で実行

## 注意点
- mcp-installerがうまくいかない場合は手動設定が必要
- APIキーやアクセストークンなどの機密情報をチャットに入力しない
- 実行権限のプロンプトが表示される場合がある（セキュリティ上仕方ない）

## まとめ
mcp-installerは、MCPサーバーの導入を劇的に簡単にするメタツール。会話ベースで様々なMCPサーバーを導入でき、AIの能力拡張を身近にする。