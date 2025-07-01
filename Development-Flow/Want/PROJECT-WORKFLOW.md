# プロジェクト作成ワークフロー

## 新規プロジェクト開始の手順

### ステップ1: Wantファイルの作成
1. `Want`ディレクトリに移動
2. `_TEMPLATE_Want-Item.md`をコピー
3. ファイル名を以下の形式にリネーム：
   ```
   YYYY-MM-DD_[カテゴリ]_[プロジェクト名].md
   ```
4. テンプレートの内容を具体的なプロジェクト情報で更新

### ステップ2: Development-Docsフォルダの作成
プロジェクトが「計画中」ステータスになったら：

1. `Development-Docs`ディレクトリに新しいフォルダを作成
2. 適切なテンプレートをコピー：
   - **Webアプリ**: `_TEMPLATES/Web-App-Template/*`
   - **ツール開発**: `_TEMPLATES/Tool-Template/*`
   - **コンテンツ作成**: `_TEMPLATES/Content-Template/*`
   - **API開発**: `_TEMPLATES/API-Template/*`

### ステップ3: プロジェクト情報の更新
1. `README.md`でプロジェクト概要を記載
2. `Requirements.md`で詳細な要件を定義
3. 必要に応じて他のドキュメントも作成・更新

### ステップ4: 相互リンクの設定
1. WantファイルからDevelopment-Docsへのリンクを追加
2. Development-DocsからWantファイルへのリンクを追加
3. 関連する他のプロジェクトとのリンクも設定

## クイック作成コマンド例

### Obsidianテンプレート活用
1. **Templates プラグイン**を使用
2. 以下のテンプレートを設定：

#### Want項目作成テンプレート
```markdown
<%* 
const projectName = await tp.system.prompt("プロジェクト名を入力してください");
const category = await tp.system.suggester(
  ["Web-App", "Tool", "Tutorial", "API", "Game", "Other"], 
  ["Web-App", "Tool", "Tutorial", "API", "Game", "Other"]
);
const date = tp.date.now("YYYY-MM-DD");
-%>
# <% projectName %>

作成日: <% date %>
カテゴリ: <% category %>
```

#### Development-Docs作成自動化
```bash
#!/bin/bash
# create-project.sh

PROJECT_NAME="$1"
PROJECT_TYPE="$2"

if [ -z "$PROJECT_NAME" ] || [ -z "$PROJECT_TYPE" ]; then
    echo "使用方法: ./create-project.sh <プロジェクト名> <プロジェクトタイプ>"
    echo "プロジェクトタイプ: Web-App, Tool, Content, API"
    exit 1
fi

# Development-Docsにフォルダ作成
mkdir -p "Development-Docs/$PROJECT_NAME"

# テンプレートをコピー
cp -r "Development-Docs/_TEMPLATES/${PROJECT_TYPE}-Template/"* "Development-Docs/$PROJECT_NAME/"

echo "プロジェクト '$PROJECT_NAME' を作成しました"
```

## プロジェクト管理のベストプラクティス

### タグ付けルール
```yaml
tags:
  - want                    # 必須：Wantディレクトリの項目
  - [カテゴリ名]            # プロジェクトカテゴリ
  - [技術スタック]          # 使用技術
  - [優先度]               # priority-high/medium/low
  - [ステータス]           # status-idea/planning/development/completed
```

### ファイル命名規則
- **Want**: `YYYY-MM-DD_[カテゴリ]_[プロジェクト名].md`
- **Development-Docs**: `[プロジェクト名]/[ドキュメント種別].md`
- **画像**: `assets/[プロジェクト名]/[種別]/[ファイル名]`

### 定期レビューフロー

#### 週次レビュー（毎週月曜日）
1. **新規アイデアの整理**
   - 追加されたWant項目を確認
   - カテゴリ・タグの整理
   - 優先度の設定

2. **進行中プロジェクトの確認**
   - ステータス更新
   - 進捗記録
   - ブロッカーの特定

#### 月次レビュー（毎月第1週）
1. **完成プロジェクトの整理**
   - 成果物のリンク追加
   - 学びの記録
   - アーカイブ処理

2. **長期停滞プロジェクトの見直し**
   - 継続判断
   - 優先度再評価
   - 必要に応じてキャンセル

## Obsidianプラグイン活用

### 推奨プラグイン
1. **Templater**: テンプレート自動化
2. **Tag Wrangler**: タグ管理
3. **Kanban**: プロジェクト進捗管理
4. **Calendar**: 日付ベースの管理
5. **Graph Analysis**: プロジェクト間の関連性可視化

### Kanbanボード設定例
```
## プロジェクト管理ボード

### アイデア
- Want項目のステータスが「アイデア」のもの

### 計画中  
- Want項目のステータスが「計画中」のもの
- Development-Docsの要件定義段階

### 開発中
- 実装・制作作業中のプロジェクト

### レビュー中
- 完成に近く、最終確認中のプロジェクト

### 完成
- 完了したプロジェクト
```

## トラブルシューティング

### よくある問題と解決方法

#### 問題1: プロジェクトが多すぎて管理しきれない
**解決方法**:
- 優先度を明確にする
- 同時進行プロジェクト数を制限（推奨：3-5個）
- 定期的な見直しでキャンセル判断

#### 問題2: ドキュメントが更新されない
**解決方法**:
- 開発と並行してドキュメント更新
- テンプレートにチェックリスト追加
- 週次レビューで更新状況確認

#### 問題3: リンクが壊れる
**解決方法**:
- 相対パスでのリンク使用
- ファイル移動時のリンク更新確認
- Link検証プラグインの活用

## 自動化・効率化のアイデア

### GitHub Actions連携
```yaml
name: Project Sync
on:
  push:
    paths:
      - 'Want/**'
      - 'Development-Docs/**'

jobs:
  sync:
    runs-on: ubuntu-latest
    steps:
      - name: プロジェクト状況の自動更新
        run: |
          # Wantファイルのステータス変更を検知
          # Development-Docsの対応フォルダ自動作成
```

### API連携
- **Notion API**: プロジェクト情報の同期
- **GitHub API**: リポジトリ作成の自動化
- **Discord/Slack**: 進捗通知の自動化

---

**作成日**: {{date}}  
**最終更新**: {{date}}
