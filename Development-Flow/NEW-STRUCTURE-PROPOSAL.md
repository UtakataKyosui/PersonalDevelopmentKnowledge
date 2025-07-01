# プロジェクト別ディレクトリ構造

## 新しい構造概要

```
Projects/                           # 各プロジェクトの専用ディレクトリ
├── _TEMPLATES/                     # プロジェクトテンプレート
│   ├── Web-App-Project/
│   ├── Tool-Project/
│   ├── API-Project/
│   ├── Content-Project/
│   └── General-Project/
│
├── 2025-06-18_Web-App_タスク管理ツール/    # 個別プロジェクト
│   ├── 00_PROJECT-OVERVIEW.md      # プロジェクト概要（Want情報も含む）
│   ├── 01_Requirements.md          # 要件定義
│   ├── 02_Architecture.md          # 設計・アーキテクチャ
│   ├── 03_Development-Log.md       # 開発ログ
│   ├── 04_Testing.md              # テスト
│   ├── 05_Deployment.md           # デプロイ
│   ├── assets/                    # プロジェクト専用アセット
│   │   ├── images/
│   │   ├── diagrams/
│   │   └── mockups/
│   └── docs/                      # 追加ドキュメント
│
├── 2025-06-20_Tool_CLI-Generator/
│   ├── 00_PROJECT-OVERVIEW.md
│   ├── 01_Requirements.md
│   ├── 02_Tool-Design.md
│   ├── 03_Development-Log.md
│   ├── 04_Package-Strategy.md
│   ├── assets/
│   └── examples/                  # ツール使用例
│
└── 2025-06-22_Content_DevContainer活用ガイド/
    ├── 00_PROJECT-OVERVIEW.md
    ├── 01_Content-Strategy.md
    ├── 02_Writing-Plan.md
    ├── 03_Development-Log.md
    ├── 04_Publishing-Plan.md
    ├── assets/
    └── drafts/                    # 原稿ドラフト
```

## ファイル命名規則

### プロジェクトディレクトリ名
```
YYYY-MM-DD_[カテゴリ]_[プロジェクト名]/
```

### プロジェクト内ファイル
- `00_PROJECT-OVERVIEW.md` - プロジェクト概要（必須）
- `01_` - 要件・戦略関連
- `02_` - 設計・計画関連  
- `03_` - 開発・制作ログ
- `04_` - テスト・品質管理
- `05_` - デプロイ・公開
- `99_` - その他・参考資料

## メリット

### 🎯 プロジェクト集約
- 関連ファイルが一箇所にまとまる
- アセット（画像、図表）もプロジェクト内で管理
- プロジェクト固有の設定やスクリプトも同居可能

### 📊 進捗管理
- フォルダレベルでプロジェクト状況が把握できる
- ファイル番号で作業の順序が明確
- プロジェクト間の切り替えが容易

### 🔍 検索・発見
- プロジェクト名での検索が効率的
- 関連ドキュメントがまとまって表示される
- Obsidianのフォルダ機能をフル活用

### 🚀 拡張性
- プロジェクトタイプに応じたカスタマイズが容易
- 将来的なプロジェクトアーカイブも簡単
- 外部ツール（Git等）との連携もスムーズ
