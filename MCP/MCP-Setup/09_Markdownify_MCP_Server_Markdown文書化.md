# MCPのトリセツ #9: Markdownify MCP Server - WebページやPDFをMarkdown文書化

## 記事情報
- **著者**: ながたく (takna)
- **URL**: https://zenn.dev/takna/articles/mcp-server-tutorial-09-markdownfy
- **シリーズ**: MCPのトリセツ

## 概要
Markdownify MCPサーバーを使用してウェブページやPDFをAIとの会話でMarkdown文書化し、リサーチや情報収集を効率化する方法を解説。様々なファイル形式に対応した強力な変換ツール。

## Markdownify MCP Serverでできること

### 複数のファイルタイプをMarkdownに変換
- **PDF、画像、オーディオ（転写付き）、docx、XLSX、PPTX**

### WebコンテンツをMarkdownに変換
- **YouTube字幕**
- **Bing検索結果**
- **一般的なWebページ**

### 既存のMarkdownファイルを取得

## プロンプトサンプル

### 1. 基本的なURLからMarkdownへの変換
```
以下URLの内容をMarkdownに変換して：
https://example.com/blog/article-123
```

### 2. 複数のWebページを一括変換
```
以下の複数のURLをMarkdownに変換し、それぞれのコンテンツを見出しで区切って：
- https://site1.com/article1
- https://site2.com/article2
- https://site3.com/article3
```

### 3. 特定の要素に焦点を当てた変換
```
次の技術ブログから、コードスニペットと主要な見出しだけを抽出し、Markdownに変換して：
https://tech-blog-example.com/tutorial/javascript-basics
```

### 4. フォーマット指定付き変換
```
以下のニュースサイトの記事を以下の条件でMarkdownに変換して：
- 見出しは ## レベルで統一
- 引用部分は > でマーク
- リンクはそのまま保持
- 画像は ![キャプション](URL) 形式に統一
URL: https://news-example.com/technology/latest-trends
```

### 5. コンテンツフィルタリング付き変換
```
以下URLからAIに関連する内容だけを抽出してMarkdownに変換して。
関連性の低いセクションは除外して
https://news.ycombinator.com/best
```

### 6. PDFからMarkdownへの変換
```
添付のPDFをMarkdownに変換して。
目次構造を保持し、表はMarkdown形式のテーブルとして変換して
```

### 7. YouTube動画の字幕抽出
```
以下のYouTube動画の字幕をMarkdown形式で抽出して。
タイムスタンプを含め、主要なポイントには見出しを付けて：
https://www.youtube.com/watch?v=example12345
```

### 8. 技術文書の整形済み変換
```
次の技術ドキュメントをMarkdownに変換し、以下の形式に整えて：
- コードブロックは言語指定付きで
- API エンドポイントは表形式で整理
- パラメータ説明はリスト形式で
- 重要な警告は強調表示
URL: https://api-docs-example.com/reference
```

## 高度な活用例

### アカデミック論文の構造化変換
```
以下の学術論文をMarkdownに変換し、次の構造を明確にして：
- 要旨（Abstract）を引用形式で
- 方法論、結果、考察を別々のセクションで
- 参考文献を番号付きリストで
- 図表に連番を振って参照しやすく
URL: https://academic-journal.edu/paper/12345
```

### SEO分析付き変換
```
次のランディングページをMarkdownに変換し、SEO観点からの分析を追加して：
- H1, H2 の見出し構造
- メタタグ情報
- キーワード密度
- 内部リンク・外部リンクの数と質
URL: https://business-site.com/services
```

### ニュースレター作成用フォーマット
```
以下の複数のニュース記事を取得し、週刊ニュースレター形式のMarkdownに変換して：
- トップニュース（3件）を冒頭に要約
- カテゴリ別に記事をグループ化
- 各記事に 「続きを読む」 リンクを追加
- 最後に今週の重要なイベントカレンダーを表形式で
```

### 製品比較表の作成
```
以下の複数の製品レビューページから情報を抽出し、Markdown形式の比較表を作成して：
- 製品名、価格、主な機能、長所、短所の列を含む
- 価格帯で製品を並べ替え
- 各製品の総合評価（5段階）を追加
```

## インストールと設定

### インストール手順

1. **インストールディレクトリに移動**
```bash
cd ~/tools/mcp-server
# 場所はお好みで
```

2. **リポジトリをクローン**
```bash
git clone git@github.com:zcaceres/markdownify-mcp.git
```

3. **ディレクトリに移動**
```bash
cd markdownify-mcp
```

4. **インストール**
```bash
pnpm install
```

5. **ビルド**
```bash
pnpm run build
```

### Claude Desktop設定
Claude Desktopの設定ファイルに追加:

```json
{
  "mcpServers": {
    "markdownify": {
      "command": "node",
      "args": [
        "{ABSOLUTE PATH TO FILE HERE}/dist/index.js"
      ],
      "env": {
        "UV_PATH": "/path/to/uv"
      }
    }
  }
}
```

**⚠️ 重要**: 
- `args`には`index.js`の絶対パスを設定
- `~`でホームディレクトリを表すとエラーになるので注意
- `uv`のパスはデフォルト設定なら不要（必要な場合は`which uv`で確認）

## 主な特徴とメリット
- **複数の情報源を統一されたMarkdown形式で管理**
- **情報の整理・構造化が容易**で後からの参照や編集が効率化
- **AIとの対話でコンテンツの変換だけでなく要約や分析も同時実行**
- **様々なフォーマット指定やフィルタリングオプション**で必要な情報だけを抽出
- **他のMCPサーバーとの組み合わせ**で情報収集から整理、分析、保存までのワークフロー自動化

## 使い分けについて
Fetch MCP、Firecrawl MCP、Markdownify MCPの比較と実践的な使い分けについては、「ウェブの情報を取得するMCPの使い分け」を参照。

## まとめ
Markdownify MCPサーバーは、様々な形式のコンテンツをMarkdown形式に変換できる強力なツール。研究、コンテンツ制作、技術文書作成、情報整理などの作業で大きな時間節約と効率化を実現。他のMCPサーバーと組み合わせることで、包括的な情報処理ワークフローの構築が可能。