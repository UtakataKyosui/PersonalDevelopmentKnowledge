# MCPのトリセツ #4: mcp-pandoc - AIでドキュメント形式を変換

## 記事情報
- **著者**: ながたく (takna)
- **URL**: https://zenn.dev/takna/articles/mcp-server-tutorial-04-pandoc
- **シリーズ**: MCPのトリセツ

## 概要
Pandocという強力なドキュメント変換ツールをAIから操作できるようになるMCPサーバー。テキストをさまざまな形式（txt、docx、epub、html、markdownなど）に変換可能。

## mcp-pandocでできること
- Markdown を Wordドキュメント（docx）に変換
- Markdown を ePub に変換して電子書籍化
- Markdown をPDF変換（※開発中・2025年3月時点）
- Word文書を Markdown に変換
- プレーンテキストを HTMLに変換

## 使用例（プロンプト）

### 1. マークダウンからWord文書作成
```
これを docx に変換して、デスクトップフォルダにファイル名 "SampleDoc" で保存して。
```

### 2. マークダウンからHTML作成
```
html 形式に変換し、見やすいレイアウトとデザインにして、デスクトップフォルダに sample.html として保存して

この Markdown形式の技術解説を、シンタックスハイライトとレスポンシブデザインを適用したHTMLに変換して、~/Projects/docs/technical-guide.html として保存して
```

### 3. マークダウンからPDF作成
```
PDF に変換して、元ファイルと同じ場所に保存して
```

### 4. ビジネス文書作成
```
この会議メモをプロフェッショナルなWord文書に変換して、目次と見出しを適切に設定し、~/Documents/Business/meeting-report.docx として保存してください。
```

### 5. 電子書籍作成
```
この小説をePub形式に変換し、表紙画像を追加して、章ごとに適切に区切ってください。~/Documents/Books/my-novel.epub に保存してください。
```

### 6. 既存文書ファイルの変換
```
/Users/yourname/Documents/report.md ファイルを読み込んで、Word形式に変換し、同じフォルダに report.docx として保存してください。
```

### 7. YouTube MCPサーバーとの組み合わせ
```
1. このYouTube講義の内容を要約してください：https://www.youtube.com/watch?v=xxxxx
2. 要約をマークダウン形式でまとめてください
3. それをHTML形式に変換して、目次とセクション分けを追加し、~/Documents/lectures/summary.html として保存してください
```

## 使用上の注意点
- **ファイルパスの指定**: ファイル名と拡張子を含む完全なファイルパスを指定する必要
- **PDF対応状況**: PDFサポートは現在開発中の機能（2025年3月時点）
- **ディレクトリアクセス**: 保存先のディレクトリがFilesystem MCPでアクセス許可されていることを確認

## インストール手順

### 前提条件
- Filesystem MCPサーバーが事前にインストールされていること

### 1. TeX Liveのインストール（PDF変換用）

**macOS**:
```bash
brew install texlive
```

**Windows**: 
[TeX Live公式サイト](https://tug.org/texlive/)からインストーラーをダウンロード

### 2. mcp-pandocのインストール

#### 方法1: mcp-installerを使用（推奨）
```
MCPサーバー mcp-pandoc をインストールして
```

#### 方法2: 設定ファイルを直接編集
Claude Desktopの設定ファイル（`~/Library/Application Support/Claude/claude_desktop_config.json`）に追加:

```json
{
  "mcpServers": {
    "mcp-pandoc": {
      "command": "uvx",
      "args": [
        "mcp-pandoc"
      ],
      "env": {}
    }
  }
}
```

設定後、Claude Desktopを再起動。

## 活用のポイント
- AIとの会話で生まれたコンテンツを即座に各種形式で保存可能
- Filesystem MCP、YouTube MCPなど他のサーバーとの組み合わせで可能性が拡大
- ビジネス文書、技術ドキュメント、創作活動など幅広い用途に対応

## まとめ
mcp-pandocは、AIとの会話で生まれたコンテンツを様々な形式で保存・活用するための強力なツール。Markdown、Word、HTML、ePubなど多くの形式に対応し、他のMCPサーバーとの組み合わせでさらに可能性が広がる。