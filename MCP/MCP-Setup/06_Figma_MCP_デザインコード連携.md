# MCPのトリセツ #6: Figma MCP - デザインとコードを効率的に連携

## 記事情報
- **著者**: ながたく (takna)
- **URL**: https://zenn.dev/takna/articles/mcp-server-tutorial-06-figma
- **シリーズ**: MCPのトリセツ

## 概要
Figma MCPサーバーを使用してAIがFigmaのデザインデータに直接アクセスし、デザインとコードの連携を効率化する方法を解説。デザイナーとエンジニアの連携がスムーズになる。

## Figma MCPでできること
- **正確なデザイン情報の取得**: 色、サイズ、フォント、間隔などの正確な数値を取得
- **コンポーネントデータの抽出**: ボタン、カード、ナビゲーションなどのコンポーネント情報を分析
- **デザインからコードに変換**: FigmaデザインをHTML/CSS/JavaScript、Reactコンポーネントに変換
- **デザイントークンの抽出**: カラーパレット、タイポグラフィ、スペーシングを一括取得
- **レスポンシブデザインの分析**: 異なるブレークポイントでのレイアウト変化を理解
- **デザインコメントの参照**: Figmaファイル内のコメントやフィードバックを参照しながら実装

## セットアップ手順

### 1. Figma APIキーの取得
1. [Figma](https://www.figma.com)にログイン
2. 右上のアイコン → Settings
3. 左側メニュー → Accountタブ
4. 下部の「Personal access tokens」セクション → 「Create a new personal access token」
5. トークン名を入力（例: `figma-mcp-2025-03-01`）→ 「Create token」
6. 表示されたAPIキーをコピー（一度しか表示されないので安全に保存）

### 2. エディタの設定

#### Claude Desktop
`~/Library/Application Support/Claude/claude_desktop_config.json` に追加:

```json
{
  "mcpServers": {
    "figma-developer-mcp": {
      "command": "npx",
      "args": ["-y", "figma-developer-mcp", "--stdio"],
      "env": {
        "FIGMA_API_KEY": "your_figma_api_key_here"
      }
    }
  }
}
```

#### Windsurf
`~/.codeium/windsurf/mcp_config.json` に同様の設定を追加

### 3. MCPサーバーの起動
アプリケーションを再起動すると自動的にMCPサーバーが起動

手動起動の場合:
```bash
npx figma-developer-mcp --figma-api-key=your_figma_api_key_here
```

## 基本的な使い方（プロンプト）

### Figmaファイルの接続
```
Figma MCPサーバーに繋いで https://www.figma.com/file/xxxxXXXXxxxxXXXX/ProjectName
```

### デザイン要素の取得
```
このFigmaファイルのメインボタンのスタイル情報を教えて
```

### コード生成
```
このボタンをReactコンポーネントとして実装して
```

### レスポンシブ対応
```
このコンポーネントをモバイル対応にするには？
```

## 特定のエリアやコンポーネントの取得

### ノードIDの取得方法
1. Figmaデザインファイルを開く
2. 対象の要素を選択
3. ノードIDの確認:
   - ブラウザのアドレスバーで `node-id=X-Y` を確認
   - または、要素を右クリック → 「Copy/Paste as」→ 「Copy link」

### 使用例
```
Figma MCPでファイルの node-id=123-456 のコンポーネント情報を取得して

Figma MCPでこのURL（https://www.figma.com/file/...?node-id=789-012）の「Hero」セクションの実装コードを生成して
```

## 実践的な活用例

### 1. デザインシステムの分析
```
Figmaコンポーネントライブラリからカラー、タイポグラフィ、スペーシングなどのデザイントークンを抽出し、CSS変数として出力して
```

### 2. デザイン仕様をローカルファイルに保存（Figma MCP + Filesystem MCP）
```
1. Figmaファイル（https://www.figma.com/file/xxxxx）のデザイントークンを分析して
2. カラー、タイポグラフィ、スペーシングの情報をCSS変数として /Users/yourname/Projects/design-system/tokens.css ファイルに保存して
```

### 3. バリエーション含むコンポーネント生成
```
このボタンの4つの状態（通常、ホバー、アクティブ、無効）を含む Reactコンポーネントセットを生成して

このボタンコンポーネントを分析して、class-variance-authority（CVA）を使用したバリアント（サイズ: sm, md, lg、バリエーション: primary, secondary, outline, ghost）を持つ Tailwind CSS のボタンコンポーネントを実装して
```

### 4. アニメーション実装提案
```
このモーダルのトランジションアニメーションを Framer Motion で実装するには？
```

### 5. デザイン仕様書の自動生成
```
このヘッダーコンポーネントの詳細な仕様書（サイズ、色、間隔、フォントなど）をMarkdownで作成して
```

### 6. デザインの一貫性チェック
```
このデザインのカラーパレットとタイポグラフィの使用を分析し、デザインシステムから外れている部分があれば指摘して
```

## まとめ
Figma MCPを導入することで、デザインとコーディングの橋渡しがスムーズになり、開発効率が向上。デザイナーの意図を正確に反映したコード実装が容易になり、デザインの一貫性も保ちやすくなる。

特にフロントエンド開発者やUI/UXデザイナーにとって、Figma MCPは強力なアシスタント。デザインデータを直接取得できることで、情報の見落としや解釈の誤りを減らし、より質の高い実装が可能。

## 参考リンク
- [Figma API公式ドキュメント](https://www.figma.com/developers/api)
- [figma-developer-mcp GitHub リポジトリ](https://github.com/figma/figma-developer-mcp)
- [Figma開発者コミュニティ](https://www.figma.com/community)
- [AIエディタ：Windsurf公式ドキュメント](https://codeium.com/windsurf)
- [AIエディタ：Cursor公式サイト](https://cursor.sh/)