# repomix - VibeCoding必須ツール

> **原文**: [VibeCodingに必須の便利ツール「repomix」の紹介](https://zenn.dev/cryptobox/articles/c497adc7f3eed4)

## 概要

LLMに渡すためのコードコンテキストを1つのファイルにまとめてくれるツール。yamadashy氏によって制作され、Open Source Awards 2025のPowered by AI部門にノミネート。VibeCodingにおける実装計画作成に必須。

## repomixとは

### 基本機能
- **コードコンテキスト統合**: 複数ファイルを1つのファイルにまとめる
- **LLM最適化**: XML形式での構造化出力
- **柔軟な選択**: 必要なファイル・ディレクトリのみを指定可能

### 開発者・受賞歴
- **開発者**: [yamadashy](https://x.com/yamadashy)
- **GitHub**: [yamadashy/repomix](https://github.com/yamadashy/repomix)
- **受賞**: [Open Source Awards 2025](https://osawards.com/javascript/) Powered by AI部門ノミネート

## VibeCodingにrepomixが必要な理由

### 実装計画作成の重要性
- VibeCodingでは実装計画作成が必須
- 計画なしの自己実装は骨が折れる
- ハルシネーションが起きやすい
- 意図した実装から外れやすい

### 品質の高いコンテキストの重要性
- **必要最低限の情報**: 過多でも過少でもハルシネーション発生
- **構造化された情報**: LLMが理解しやすい形式
- **関連性の高い情報**: 実装に直接必要な情報に限定

## インストール・使用方法

### インストール
```bash
# npxで即座に使用
npx repomix

# グローバルインストール
npm install -g repomix
# or
yarn global add repomix
# or (macOS/Linux)
brew install repomix
```

### 基本使用例
```bash
# 特定ファイルを指定
repomix --include src/pages/index.astro,src/constants,package.json
```

## 実践例：ページネーション機能実装

### 対象ファイル選択
| ファイル | 概要 |
|---|---|
| `src/pages/index.astro` | ページネーション機能を実装したいファイル |
| `src/constants` | 定数ファイル。1ページ表示数の管理 |
| `package.json` | パッケージマネージャー・ライブラリ情報 |

### 生成されるrepomix-output.yml
```yaml
# このファイルはrepomixによって生成されたコードベースの統合表現
<file_summary>
<purpose>
リポジトリの内容をAIシステムによる分析、コードレビュー、
その他の自動化プロセスで消費しやすいようにパッケージ化
</purpose>

<file_format>
1. このサマリーセクション
2. リポジトリ情報
3. ディレクトリ構造
4. ファイルエントリ（ファイルパス + 内容）
</file_format>
</file_summary>

<directory_structure>
package.json
src/constants/common.ts
src/pages/index.astro
</directory_structure>

<files>
<file path="src/constants/common.ts">
export const PAGE_SIZE = 2;
</file>

<file path="package.json">
{
  "name": "blog-app",
  "dependencies": {
    "@astrojs/mdx": "^4.0.7",
    "astro": "^5.1.9",
    // ... 他の依存関係
  }
}
</file>

<file path="src/pages/index.astro">
---
import { getCollection } from "astro:content";
import { PAGE_SIZE } from "../constants/common";
// ... コンポーネントの実装
---
</file>
</files>
```

### LLMへの指示例
```
// repomix-output.ymlの中身をコピペ

src/pages/index.astroにページネーション機能を実装する計画書を作成して。
ページネーションコンポーネントはshadcnのものを使って
PostPreviewの下に配置するようにして。

またディレクトリ構成は以下なので参考にして
// tree -L 2 src の結果をコピペ
```

## repomixの必要性

### IDEの制限
- **Cursorの制限**: Add Contextで大量コンテキストを与えると読み飛ばし発生
- **一定規模超え**: 生成される計画書の質が目に見えて悪化
- **repomix使用**: 読み飛ばしがほとんどなく、高品質なアウトプット

### 追加メリット

#### モデル本来の力を使用可能
- **例**: Gemini 2.5 Pro（100万トークン対応）
- **Copilot制限**: 64k程度に制限
- **repomix利用**: ブラウザからフル活用可能

#### 最新モデルのいち早い利用
- IDEアップデート待ちなし
- 最新モデル発表後すぐに使用可能

#### 別リポジトリのコンテキスト統合
- OpenAPIスキーマなど他リポジトリ情報も含めて実装計画作成
- 必要な部分のみの柔軟な取捨選択

## 実装フロー

### 1. repomixでコンテキスト生成
```bash
repomix --include src/pages/index.astro,src/constants,package.json
```

### 2. 推論モデルで実装計画作成
o3などの高性能モデルにrepomix-output.ymlを渡して詳細な実装計画書を生成

### 3. エージェントに実装指示
```
上記の指示書に従って実装してください
```

CursorやCopilot Agentに実装計画書を渡して実装実行

### 4. 結果確認
想定通りにページネーションコンポーネントが配置され、機能も問題なく実装完了

## repomixの出力特徴

### 構造化情報
- **ファイルサマリー**: 目的と形式の明確化
- **ディレクトリ構造**: プロジェクト全体の把握
- **ファイル内容**: パス付きの詳細コード

### LLM最適化
- **XML形式**: 構造化されたデータ形式
- **コメント除去**: 不要な情報の削除
- **圧縮出力**: トークン効率の最適化

## まとめ

repomixは、VibeCodingにおける実装計画作成の質を劇的に向上させる必須ツール。IDEの制限を回避し、モデル本来の性能を活用して高品質な開発を実現。別リポジトリのコンテキスト統合など、柔軟な使い方により開発効率を大幅に向上させる。

## リンク
- [GitHub Repository](https://github.com/yamadashy/repomix)
- [開発者Zenn記事](https://zenn.dev/yamadashy/articles/ai-tool-repomix-5000-star)