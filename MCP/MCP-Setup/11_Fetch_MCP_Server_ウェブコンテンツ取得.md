# MCPのトリセツ #11: Fetch MCP Server - ウェブコンテンツを取得・処理

## 記事情報
- **著者**: ながたく (takna)
- **URL**: https://zenn.dev/takna/articles/mcp-server-tutorial-11-fetch
- **シリーズ**: MCPのトリセツ

## 概要
Fetch MCP Serverを使用してAIとの会話でウェブページの内容を取得・処理し、情報収集を効率化する方法を解説。Model Context Protocolの公式リファレンス実装の一つ。

## Fetch MCP Serverでできること
- **ウェブページの内容をMarkdown形式で取得**
- **長いウェブページを分割して読み込み**（チャンク読み込み）
- **生のHTMLコンテンツの取得**（オプション）
- **取得するコンテンツの長さ制限**

Fetch MCPは、シンプルで軽量な設計で基本的なウェブコンテンツ取得に特化し、単一ページの情報取得を効率的に実行。

## プロンプトサンプル

### 基本的なウェブページ取得
```
https://example.com のコンテンツを取得して

Python公式サイトの内容を要約して
```

### 長いコンテンツを分割して読む
```
https://docs.python.org/3/tutorial/index.html の内容を取得して、長い場合は続きを読みたい

前のページの続きを5000文字分読んで
```

### 特定のドキュメントから情報を抽出
```
https://github.com/modelcontextprotocol/servers のREADMEから、MCPサーバーの一覧を抽出して
```

### 生のHTMLコンテンツを取得
```
https://example.com の生のHTMLを取得して
```

### 複数ページの情報を組み合わせる
```
PythonとJavaScriptの公式ドキュメントから、それぞれの配列操作の違いを比較して
```

### 技術文書の特定部分を参照
```
Reactの公式ドキュメントからフックの使い方について説明して
```

### ニュースサイトから最新情報を取得
```
BBCニュースの最新記事を取得して要約して
```

### APIドキュメントの参照
```
OpenAI APIのドキュメントからChat Completions APIの使い方を教えて
```

## 高度なプロンプト例

### 特定の情報をピンポイントで抽出
```
https://nodejs.org/en/docs/ から最新のNode.jsバージョンとその主な機能を抽出して
```

### 技術比較のための情報収集
```
ReactとVueの公式ドキュメントからコンポーネントライフサイクルの違いを比較して表にまとめて
```

### 複数ソースからの情報統合
```
TensorFlowとPyTorchの公式ドキュメントから、初心者向けの機械学習モデル構築手順を比較して説明して
```

### 特定のコード例を探す
```
MDNのJavaScriptドキュメントから、非同期処理のベストプラクティスとコード例を探して
```

### 最新の技術トレンド調査
```
GitHubブログから最近の開発者向け新機能について情報を集めて要約して
```

## インストールと設定

### 前提条件
- uvがインストール済み（インストール方法はシリーズ初回を参照）

### Claude Desktop側の設定
Claude Desktopの設定ファイル（`claude_desktop_config.json`）に追加:

```json
{
  "mcpServers": {
    "fetch": {
      "command": "uvx",
      "args": ["mcp-server-fetch"]
    }
  }
}
```

### オプション設定

#### robots.txtを無視する場合
```json
{
  "args": ["mcp-server-fetch", "--ignore-robots-txt"]
}
```
※ユーザーからのリクエストのみ。モデル経由のリクエストは無視しない

#### User-agentをカスタマイズ
```json
{
  "args": ["mcp-server-fetch", "--user-agent=YourUserAgent"]
}
```

**例：iPhone SE3の場合**
```
--user-agent="Mozilla/5.0 (iPhone; CPU iPhone OS 16_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/16.0 Mobile/15E148 Safari/605.1"
```

### デフォルトUser-agent
- `ModelContextProtocol/1.0 (Autonomous; +https://github.com/modelcontextprotocol/servers)`
- `ModelContextProtocol/1.0 (User-Specified; +https://github.com/modelcontextprotocol/servers)`

## 主な特徴とメリット
- **AIとの会話だけでウェブページの内容を簡単に取得可能**
- **Markdown形式での取得により、AIが内容を理解しやすい**
- **長いコンテンツも分割して読み込めるチャンク機能**
- **軽量で導入が簡単**
- **他のMCPサーバーと組み合わせて使用可能**

## 使い分けについて
Fetch MCP、Firecrawl MCP、Markdownify MCPの比較と実践的な使い分けについては、「ウェブの情報を取得するMCPの使い分け」を参照。

## まとめ
Fetch MCP Serverは、ウェブコンテンツを取得・処理するシンプルで強力なツール。基本的なウェブ情報収集から高度な技術文書の参照まで、幅広い用途に対応。他のMCPサーバーとの組み合わせでさらに可能性が広がる効率的な情報収集ツール。