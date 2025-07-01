# MCPのトリセツ #13: Perplexity MCP Server - Perplexityならではの検索をAIとの会話で実行

## 記事情報
- **著者**: ながたく (takna)
- **URL**: https://zenn.dev/takna/articles/mcp-server-tutorial-13-perplexity
- **シリーズ**: MCPのトリセツ

## 概要
Perplexity MCPサーバーを使用してPerplexityの検索機能を活用し、リアルタイムのWeb全体の調査結果から最も関連する洞察を返すSonar APIをAIとの会話から呼び出す方法を解説。

## 重要な前提条件
- **有料でのみ利用可能**：Perplexity の Sonar API Key が必要
- **Perplexity Pro プラン**：毎月5ドル分の API 呼び出し枠が提供

## Perplexity MCP Serverでできること
- **リアルタイムのWeb全体の調査を実行**し、最新情報を取得
- **複雑な検索クエリを自然言語でAIに依頼**可能
- **検索結果を要約して整理された形で表示**
- **検索結果から関連情報を抽出して回答を生成**
- **Web検索による情報収集プロセスを効率化**
- **Webの最新情報に基づいた正確な回答を提供**
- **高度なChain of Thought（CoT）推論機能**でより詳細な分析（Reasoning）
- **複数情報源からの総合的な分析レポートを自動生成**（Deep Research）
- **引用元を明示した信頼性の高い回答を提供**

## プロンプトサンプル

### 基本的な検索クエリ
```
日本の最新の経済指標について調べて、現在の経済状況を分析して
```

### 特定のトピックに関する詳細情報
```
量子コンピューティングの最新の進展について、特に量子優位性の実証例を中心に詳しく教えて
```

### 技術比較
```
React と Vue.js の最新バージョンの違いを比較し、それぞれの長所と短所をまとめて
```

### ニュース分析
```
過去1週間の気候変動に関する主要ニュースをまとめ、各国の対応策を比較して
```

### 学術研究
```
認知行動療法の最新研究について調査し、うつ病治療における効果の最新エビデンスを報告して
```

### 商品調査
```
最新の高性能ノートパソコンを比較し、バッテリー寿命、性能、価格の観点から最もコスパの良い5つのモデルを選んで
```

## 構造化プロンプト例

### ユースケース調査（sonar-pro向け）
```
AIエージェントのユースケースを調査したいです。次のステップに従って調査結果を提供してください。
1. AIエージェントの主要な産業別ユースケースを10件程度、最新の情報源から調査してください
2. 各ユースケースについて以下を簡潔に説明してください：
   a. 産業/分野
   b. 具体的なユースケース
   c. 主なメリットと課題
3. 情報源を明記して、整理された形で結果を提示してください
```

### 競合分析レポート（sonar-reasoning-pro向け）
```
次のステップに従って、[製品名]と競合製品の比較分析を行ってください。
1. [製品名]の主要な競合製品を3〜5つ特定してください
2. 各製品について以下の情報を収集してください：
   a. 価格体系とプラン
   b. 主要機能と差別化ポイント
   c. ユーザーレビューと満足度評価
3. 収集した情報を基に、各競合製品の強みと弱みを分析してください
4. [製品名]が市場で優位性を確立するための戦略的推奨事項を提案してください
5. すべての情報源を引用して、整理された比較レポートを作成してください
```

### 技術トレンド予測（sonar-deep-research向け）
```
[技術分野]の今後3年間のトレンド予測を作成してください。以下の手順で進めてください：
1. 最新の研究論文、業界レポート、専門家の意見を調査し、[技術分野]における主要な技術的進歩を特定してください
2. これらの進歩が以下の領域にどのような影響を与えるか分析してください：
   a. 消費者行動と採用率
   b. ビジネスモデルと市場機会
   c. 規制環境と法的枠組み
3. 今後3年間の予測タイムラインを作成し、重要なマイルストーンを示してください
4. この分野に投資または注目している企業や投資家へのアクションアイテムを提案してください
5. すべての情報源を明記し、体系的にまとめた詳細レポートを作成してください
```

## インストールと設定

### 前提条件
- npmがインストール済み
- Perplexity Pro プランのアカウント

### 1. Sonar API Keyの取得
1. [Perplexity公式サイト](https://docs.perplexity.ai/guides/getting-started)でアカウント登録
2. 開発者ダッシュボードで「APIキーの生成」を選択
3. 発行されたAPIキーをメモ（後で設定で使用）

**注意**: Sonar APIの利用には課金が必要。Perplexity Proユーザーには毎月5ドル分の無料APIクレジットが提供。

### 2. MCP Serverのインストールと設定

#### インストール
```bash
# GitHubからクローン
git clone git@github.com:ppl-ai/modelcontextprotocol.git

# perplexity-askディレクトリを任意のMCPサーバー置き場にコピー
cd ~/tools/mcp-server/perplexity-ask/
npm install
```

#### Claude Desktop設定
`claude_desktop_config.json`に追加:

```json
{
  "mcpServers": {
    "perplexity-ask": {
      "command": "node",
      "args": [
        "インストール場所/perplexity-ask/dist/index.js"
      ],
      "env": {
        "PERPLEXITY_API_KEY": "YOUR_API_KEY_HERE"
      }
    }
  }
}
```

**⚠️ 重要**: 
- `args`には`dist/index.js`の絶対パスを指定
- ホームディレクトリは`~/`表記だとうまくいかない（絶対パスで）
- `YOUR_API_KEY_HERE`を実際のSonar API Keyに置き換え

## 利用可能なモデル

### 主なモデルの特徴
- **sonar-deep-research**: 最も高度な検索・分析能力（コンテキスト長: 128k）
- **sonar-reasoning-pro**: 推論能力に優れ、Chain of Thought（CoT）対応（コンテキスト長: 128k）
- **sonar-pro**（デフォルト）: 高度な検索機能と質問フォローアップ対応（コンテキスト長: 200k）
- **sonar**: 軽量で速い検索モデル（コンテキスト長: 128k）
- **r1-1776**: 検索機能を使わないオフラインチャットモデル（コンテキスト長: 128k）

### 検索パラメータのカスタマイズ
`index.ts`スクリプト内のAPI呼び出しを直接変更可能：

```json
{
  "model": "sonar-pro",
  "messages": [
    {
      "role": "system",
      "content": "あなたは役立つアシスタントです。最新の情報を提供してください。"
    },
    {
      "role": "user",
      "content": "東京の現在の天気はどうですか？"
    }
  ],
  "temperature": 0.7,
  "max_tokens": 2000,
  "top_p": 1,
  "stream": false
}
```

## 使用時のコツ
他のMCPが呼ばれてしまう場合があるため、「Perplexityで」と指示を付けることで適切にperplexity-ask MCPを呼び出すことができる。

## トラブルシューティング
問題が発生した場合：
- Claudeのトラブルシューティングガイドを参照
- `api@perplexity.ai`にサポートを依頼
- [GitHub Issues](https://github.com/ppl-ai/api-discussion/issues)でバグを報告

## 主な特徴とメリット
- **Web検索をAIの会話に自然に統合**し、最新情報に基づいた正確な回答
- **Reasoningモデルによる高度な推論**
- **Deep Researchによる複数情報源からの総合的な調査**
- **情報源の明示による信頼性の向上**
- **AIのハルシネーション（事実に基づかない誤った情報生成）を減少**
- **複雑な情報収集からレポート作成までのワークフローを効率化**

## 参考リンク
- [ppl-ai/modelcontextprotocol - Github](https://github.com/ppl-ai/modelcontextprotocol)
- [Integrating MCP with Perplexity's Sonar API - Perplexity](https://docs.perplexity.ai/guides/mcp-server)
- [Sonar API 料金表/各モデルの説明 - Perplexity](https://docs.perplexity.ai/guides/pricing)
- [Perplexity公式APIドキュメント](https://docs.perplexity.ai/api-reference/chat-completions)

## まとめ
Perplexity MCP Serverは、PerplexityのSonar APIをAIとの会話から呼び出すコネクタ。最新のWeb情報にアクセスしながら、高度な推論能力と複数情報源からの総合的な調査機能を活用可能。情報源の明示により信頼性が高く、AIのハルシネーションを減少させる効果的なツール。