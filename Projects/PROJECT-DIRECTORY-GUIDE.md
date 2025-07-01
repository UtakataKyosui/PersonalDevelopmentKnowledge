# 🚀 プロジェクト別ディレクトリシステム - 使い方ガイド

## 📋 新しいディレクトリ構造

### 基本構造
```
Projects/                           # 全プロジェクトの管理ディレクトリ
├── _TEMPLATES/                     # プロジェクトタイプ別テンプレート
│   ├── Web-App-Project/            # Webアプリ開発用
│   ├── Tool-Project/               # CLI・ライブラリ開発用
│   ├── Content-Project/            # コンテンツ制作用
│   └── API-Project/                # API・バックエンド開発用（準備中）
│
└── [実際のプロジェクト]/
    ├── 00_PROJECT-OVERVIEW.md      # プロジェクト概要（必須）
    ├── 01_Requirements.md          # 要件・戦略
    ├── 02_Architecture.md          # 設計・計画
    ├── 03_Development-Log.md       # 開発・制作ログ
    ├── 04_Testing.md              # テスト・品質管理
    ├── 05_Deployment.md           # デプロイ・公開
    ├── assets/                    # プロジェクト専用素材
    │   ├── images/                # 画像・スクリーンショット
    │   ├── diagrams/              # 図表・ダイアグラム
    │   └── mockups/               # デザインモックアップ
    └── [プロジェクト固有フォルダ]/   # examples/, drafts/ など
```

## 🎯 メリット・特徴

### ✅ プロジェクト集約管理
- **関連ファイル一箇所**: すべての関連ドキュメント・アセットが一つのフォルダに
- **コンテキスト維持**: プロジェクト切り替え時の思考の断絶を最小化
- **外部ツール連携**: Git管理、IDE連携がプロジェクト単位で可能

### ✅ 明確な進行管理
- **番号付きファイル**: 作業の順序が視覚的に分かりやすい
- **ステータス追跡**: フォルダレベルでプロジェクト状況を把握
- **履歴管理**: プロジェクトの発展過程が時系列で確認可能

### ✅ タイプ別最適化
- **専門テンプレート**: Web-App、Tool、Content、APIそれぞれに特化
- **ワークフロー最適化**: プロジェクトタイプに応じた効率的な進行
- **品質保証**: タイプ別のベストプラクティスを組み込み

## 🚀 使い始め方

### ステップ1: 新しいプロジェクト作成
```bash
# 1. Projectsディレクトリに移動
cd Projects/

# 2. プロジェクトディレクトリ作成
mkdir "2025-06-18_Web-App_タスク管理ツール"

# 3. 適切なテンプレートをコピー
cp -r "_TEMPLATES/Web-App-Project/"* "2025-06-18_Web-App_タスク管理ツール/"

# 4. テンプレートをカスタマイズ
cd "2025-06-18_Web-App_タスク管理ツール"
# 00_PROJECT-OVERVIEW.md を編集してプロジェクト情報を記入
```

### ステップ2: プロジェクト情報の設定
1. **00_PROJECT-OVERVIEW.md** を開く
2. `{{プロジェクト名}}` 等のプレースホルダーを実際の情報に置換
3. 基本情報（技術スタック、期間、目標）を記入
4. ステータスを「アイデア」から「計画中」に更新

### ステップ3: 段階的にドキュメント作成
1. **要件・戦略フェーズ**: `01_Requirements.md` 詳細化
2. **設計フェーズ**: `02_Architecture.md` 作成
3. **開発フェーズ**: `03_Development-Log.md` で進捗記録
4. **完成フェーズ**: `04_Testing.md`, `05_Deployment.md` 完成

## 📁 プロジェクトタイプ別ガイド

### 🌐 Web-App-Project
**対象**: React/Vue/Next.js等のWebアプリケーション

**特徴**:
- フロントエンド・バックエンド両方対応
- UI/UX設計からデプロイまで包括
- ユーザー認証、データベース設計含む

**主要ファイル**:
- `01_Requirements.md` - 機能要件・非機能要件
- `02_Architecture.md` - システム設計・技術選定
- `assets/mockups/` - UI デザインモックアップ

### 🛠️ Tool-Project
**対象**: CLI、ライブラリ、VS Code拡張等

**特徴**:
- コマンド設計・API設計重視
- パッケージング・配布戦略含む
- ユーザビリティ・開発者体験重視

**主要ファイル**:
- `02_Tool-Design.md` - CLI/API設計詳細
- `04_Package-Strategy.md` - 配布・リリース戦略
- `examples/` - 使用例・サンプルコード

### 📝 Content-Project
**対象**: ブログ記事、チュートリアル、技術書

**特徴**:
- コンテンツ戦略・ターゲット読者分析
- 執筆計画・品質管理プロセス
- プロモーション・マーケティング戦略

**主要ファイル**:
- `01_Content-Strategy.md` - コンテンツ戦略・競合分析
- `02_Writing-Plan.md` - 詳細執筆計画
- `drafts/` - 原稿ドラフト・作業中ファイル

### 🔌 API-Project（準備中）
**対象**: REST API、GraphQL、マイクロサービス

**特徴**:
- API設計・認証・認可システム
- データベース設計・パフォーマンス最適化
- セキュリティ・スケーラビリティ対応

## 🔄 Wantディレクトリとの連携

### 既存Wantシステムとの併用
```markdown
# Wantディレクトリ（既存）
Want/
├── README.md
├── _TEMPLATE_Want-Item.md
├── PROJECT-WORKFLOW.md
└── 2025-06-18_Web-App_アイデアA.md    # アイデア段階

# ↓ プロジェクト化決定時 ↓

# Projectsディレクトリ（新規）
Projects/
└── 2025-06-18_Web-App_アイデアA/      # 実装段階
    ├── 00_PROJECT-OVERVIEW.md        # Want情報を統合
    └── ...
```

### 移行・連携フロー
1. **Wantでアイデア管理**: 初期アイデアはWantディレクトリで管理
2. **プロジェクト化判断**: アイデアが具体化したらProjectsに移行
3. **相互リンク設定**: WantファイルからProjectsフォルダへのリンク追加
4. **ステータス同期**: 両方のファイルでステータスを同期更新

## 📊 管理・運用のベストプラクティス

### ファイル命名規則
```
# プロジェクトディレクトリ
YYYY-MM-DD_[カテゴリ]_[プロジェクト名]/

# 例
2025-06-18_Web-App_タスク管理ツール/
2025-06-20_Tool_CLI-Generator/
2025-06-22_Content_DevContainer活用ガイド/
```

### タグ付け戦略
```yaml
# プロジェクト共通タグ
tags:
  - project                  # 必須：プロジェクト識別
  - web-app/tool/content     # プロジェクトタイプ
  - react/rust/writing       # 主要技術・分野
  - priority-high            # 優先度
  - status-development       # 現在状況
```

### 定期レビュー・メンテナンス
#### 週次レビュー（毎週月曜日）
- [ ] 各プロジェクトのステータス確認
- [ ] 開発ログの更新状況チェック
- [ ] 次週の作業計画更新

#### 月次レビュー（毎月第1週）
- [ ] 完成プロジェクトのアーカイブ
- [ ] 長期停滞プロジェクトの見直し
- [ ] テンプレートの改善検討

## 🎯 旧システムからの移行

### 段階的移行戦略
1. **新規プロジェクト**: 今後はProjectsディレクトリを使用
2. **既存プロジェクト**: 活発なものから順次移行
3. **アーカイブ**: 完了済みはそのまま保持

### 移行作業の例
```bash
# 既存のDevelopment-Docsプロジェクトを移行
mkdir "Projects/2025-06-15_MCP-サーバー開発"
cp -r "Development-Docs/MCP-Server/"* "Projects/2025-06-15_MCP-サーバー開発/"

# ファイル名を新形式にリネーム
mv README.md 00_PROJECT-OVERVIEW.md
mv Requirements.md 01_Requirements.md
# ...
```

## 🛠️ Obsidianプラグイン活用

### 推奨プラグイン設定
```json
{
  "templater": {
    "template_folder": "Projects/_TEMPLATES",
    "auto_jump_to_cursor": true
  },
  "kanban": {
    "project_boards": "Projects/",
    "status_field": "status"
  },
  "tag_wrangler": {
    "project_tags": ["project", "web-app", "tool", "content"]
  }
}
```

### Kanbanボード活用例
```
## プロジェクト進捗ボード

### 📋 企画中
- 2025-06-18_Content_Next.js完全ガイド
- 2025-06-20_Tool_Git-Workflow-Helper

### 🚧 開発中
- 2025-06-15_Web-App_タスク管理ツール
- 2025-06-17_Tool_CLI-Generator

### 📝 レビュー中
- 2025-06-10_Content_DevContainer基礎

### ✅ 完了
- 2025-06-05_Tool_Markdown-Converter
- 2025-06-01_Content_React基礎講座
```

## 💡 実践的な使用例

### 例1: Webアプリプロジェクトの進行
```
2025-06-18_Web-App_タスク管理ツール/
├── 00_PROJECT-OVERVIEW.md          # ✅ プロジェクト基本情報
├── 01_Requirements.md              # ✅ 要件定義完了
├── 02_Architecture.md              # 🚧 設計中
├── 03_Development-Log.md           # 📝 開発ログ記録中
├── 04_Testing.md                   # ⏳ 未着手
├── 05_Deployment.md                # ⏳ 未着手
├── assets/
│   ├── images/
│   │   ├── user-flow.png          # ユーザーフロー図
│   │   └── wireframe.png          # ワイヤーフレーム
│   └── mockups/
│       └── dashboard-design.figma  # デザインファイル
└── prototype/                      # プロトタイプコード
    ├── src/
    └── package.json
```

### 例2: CLIツールプロジェクトの進行
```
2025-06-20_Tool_File-Organizer/
├── 00_PROJECT-OVERVIEW.md          # ✅ ツール概要・目標
├── 01_Requirements.md              # ✅ 機能要件
├── 02_Tool-Design.md               # ✅ CLI設計・API設計
├── 03_Development-Log.md           # 🚧 実装中
├── 04_Package-Strategy.md          # 📝 配布戦略計画中
├── assets/
│   └── diagrams/
│       └── command-structure.svg   # コマンド構造図
└── examples/                       # 使用例
    ├── basic-usage.md
    └── advanced-usage.md
```

### 例3: コンテンツプロジェクトの進行
```
2025-06-22_Content_TypeScript実践ガイド/
├── 00_PROJECT-OVERVIEW.md          # ✅ コンテンツ企画
├── 01_Content-Strategy.md          # ✅ 戦略・ターゲット分析
├── 02_Writing-Plan.md              # ✅ 執筆計画
├── 03_Development-Log.md           # 📝 執筆進捗
├── 04_Publishing-Plan.md           # 📝 公開・プロモーション計画
├── assets/
│   ├── images/
│   │   ├── typescript-basics.png
│   │   └── advanced-types.png
│   └── code-samples/               # サンプルコード
│       ├── basic/
│       └── advanced/
└── drafts/                         # 執筆中ドラフト
    ├── chapter-01.md
    ├── chapter-02.md
    └── outline.md
```

## 🔧 トラブルシューティング

### よくある問題と解決方法

#### 問題1: テンプレートが多すぎて選択に迷う
**解決方法**:
- 主要な目的で分類：作る（Web-App/Tool） vs 書く（Content）
- 迷った場合はGeneral-Projectテンプレートを作成予定
- 途中でタイプ変更も可能（ファイル追加・削除で調整）

#### 問題2: プロジェクトディレクトリ名が長い
**解決方法**:
- エイリアス・ショートカットの活用
- IDEのプロジェクト機能でブックマーク
- 必要に応じて短縮版ディレクトリ名も併用

#### 問題3: 既存のWantシステムとの重複
**解決方法**:
- Wantは「アイデア段階」、Projectsは「実装段階」として使い分け
- プロジェクト化時にWantファイルにProjectsへのリンクを追加
- 段階的移行で混乱を最小化

## 🎊 次のステップ

### 今すぐできること
1. **新規プロジェクト**: 次のプロジェクトでProjectsディレクトリを試用
2. **既存プロジェクト**: 一つ選んで新形式に移行
3. **テンプレート改善**: 使いながら自分用にカスタマイズ

### 今後の拡張予定
- [ ] **API-Projectテンプレート**: バックエンド特化テンプレート
- [ ] **General-Projectテンプレート**: 汎用プロジェクト用
- [ ] **自動化スクリプト**: プロジェクト作成・移行の自動化
- [ ] **Obsidianプラグイン**: 専用プラグインの検討

---

**これで、プロジェクト毎のディレクトリ分けシステムの準備が完了しました！** 🎉

早速、新しいプロジェクトでこのシステムを試してみてください。何か質問や改善要望があれば、お気軽にお聞かせください。
