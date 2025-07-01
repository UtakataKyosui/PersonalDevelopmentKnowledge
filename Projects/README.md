# Projects - プロジェクト管理ディレクトリ

## 📋 このディレクトリについて

`Projects`ディレクトリは、各プロジェクトを個別のフォルダで管理する新しいシステムです。アイデアから実装、完成まで、プロジェクトに関連するすべてのファイルを一箇所にまとめて効率的に管理できます。

## 🎯 システムの特徴

### プロジェクト集約管理
- **一つのフォルダ、すべての関連ファイル**: ドキュメント、アセット、設定すべてがプロジェクト単位で整理
- **外部ツール連携**: Git、IDE、デプロイツールとの連携がプロジェクト単位で簡単
- **コンテキスト維持**: プロジェクト間の切り替えがスムーズ

### タイプ別最適化
- **Web-App-Project**: Webアプリケーション開発用
- **Tool-Project**: CLI・ライブラリ開発用  
- **Content-Project**: ブログ・チュートリアル制作用
- **API-Project**: バックエンドAPI開発用（準備中）

## 📁 ディレクトリ構造

```
Projects/
├── README.md                           # このファイル
├── PROJECT-DIRECTORY-GUIDE.md          # 詳細使い方ガイド
├── _TEMPLATES/                         # プロジェクトテンプレート集
│   ├── Web-App-Project/
│   ├── Tool-Project/
│   ├── Content-Project/
│   └── API-Project/ (準備中)
│
└── [個別プロジェクト]/                  # 実際のプロジェクト
    ├── 00_PROJECT-OVERVIEW.md          # プロジェクト概要（必須）
    ├── 01_Requirements.md              # 要件・戦略
    ├── 02_Architecture.md              # 設計・計画  
    ├── 03_Development-Log.md           # 開発・制作ログ
    ├── 04_Testing.md                   # テスト・品質管理
    ├── 05_Deployment.md               # デプロイ・公開
    ├── assets/                        # プロジェクト専用素材
    │   ├── images/                    # 画像・スクリーンショット
    │   ├── diagrams/                  # 図表・ダイアグラム
    │   └── mockups/                   # デザインモックアップ  
    └── [プロジェクト固有]/              # examples/, drafts/ など
```

## 🚀 クイックスタート

### 1. 新しいプロジェクト作成
```bash
# プロジェクトフォルダ作成
mkdir "Projects/2025-06-18_Web-App_新プロジェクト"

# 適切なテンプレートをコピー
cp -r "Projects/_TEMPLATES/Web-App-Project/"* "Projects/2025-06-18_Web-App_新プロジェクト/"

# プロジェクト情報をカスタマイズ
# 00_PROJECT-OVERVIEW.md を編集
```

### 2. プロジェクト情報の設定
- `00_PROJECT-OVERVIEW.md` でプロジェクト基本情報を記入
- 技術スタック、期間、目標を明確化
- ステータスを「アイデア」→「計画中」→「開発中」と更新

### 3. 段階的ドキュメント作成
- **企画段階**: `01_Requirements.md` で要件定義
- **設計段階**: `02_Architecture.md` で設計・技術選定
- **開発段階**: `03_Development-Log.md` で進捗記録
- **完成段階**: `04_Testing.md`, `05_Deployment.md` で仕上げ

## 📊 プロジェクトタイプ別の特徴

### 🌐 Web-App-Project
**対象**: React, Vue, Next.js等のWebアプリケーション  
**特徴**: フロントエンド・バックエンド両方、UI/UX設計、データベース設計  
**主要ファイル**: Requirements, Architecture, UI/UX Design

### 🛠️ Tool-Project  
**対象**: CLI、ライブラリ、VS Code拡張等  
**特徴**: コマンド設計、API設計、パッケージング・配布戦略  
**主要ファイル**: Tool-Design, Package-Strategy, Examples

### 📝 Content-Project
**対象**: ブログ記事、チュートリアル、技術書  
**特徴**: コンテンツ戦略、執筆計画、プロモーション戦略  
**主要ファイル**: Content-Strategy, Writing-Plan, Drafts

## 🔄 Wantディレクトリとの連携

### 役割分担
- **Want**: アイデア・企画段階の管理
- **Projects**: 実装・制作段階の管理

### 連携フロー
1. Wantでアイデアを記録・育成
2. 具体化したらProjectsに移行
3. 相互リンクで連携維持
4. ステータス同期

## 🎯 管理のベストプラクティス

### 命名規則
```
YYYY-MM-DD_[カテゴリ]_[プロジェクト名]/

例:
2025-06-18_Web-App_タスク管理ツール/
2025-06-20_Tool_CLI-Generator/
2025-06-22_Content_TypeScript実践ガイド/
```

### タグ戦略
```yaml
tags:
  - project           # 必須：プロジェクト識別
  - web-app          # プロジェクトタイプ
  - react            # 主要技術
  - priority-high    # 優先度
  - status-development # 現在状況
```

### 定期レビュー
- **週次**: ステータス確認、進捗記録
- **月次**: 完成プロジェクトのアーカイブ、停滞プロジェクトの見直し

## 🛠️ ツール連携

### Obsidianプラグイン
- **Templater**: テンプレート自動適用
- **Kanban**: 進捗可視化
- **Tag Wrangler**: タグ管理
- **Calendar**: 時系列管理

### 外部ツール
- **Git**: プロジェクト単位でのバージョン管理
- **IDE**: プロジェクトフォルダ単位でのワークスペース
- **CI/CD**: プロジェクト単位でのデプロイメント

## 📈 成功事例・使用例

### Webアプリ開発
```
2025-06-15_Web-App_タスク管理/
├── 00_PROJECT-OVERVIEW.md    # ✅ 完了
├── 01_Requirements.md        # ✅ 完了  
├── 02_Architecture.md        # 🚧 作業中
├── assets/mockups/           # UI設計ファイル
└── prototype/               # プロトタイプコード
```

### CLI ツール開発
```
2025-06-10_Tool_File-Organizer/
├── 00_PROJECT-OVERVIEW.md    # ✅ 完了
├── 02_Tool-Design.md         # ✅ 設計完了
├── 03_Development-Log.md     # 📝 実装中
└── examples/                # 使用例・サンプル
```

### コンテンツ制作
```
2025-06-05_Content_React基礎講座/
├── 00_PROJECT-OVERVIEW.md    # ✅ 完了
├── 01_Content-Strategy.md    # ✅ 戦略決定
├── 03_Development-Log.md     # 📝 執筆中
└── drafts/                  # 原稿ドラフト
```

## 📚 詳細ガイド

### 詳しい使い方
- [[PROJECT-DIRECTORY-GUIDE]] - 詳細な使い方ガイド
- [[Projects/_TEMPLATES/Web-App-Project/00_PROJECT-OVERVIEW]] - Webアプリテンプレート
- [[Projects/_TEMPLATES/Tool-Project/00_PROJECT-OVERVIEW]] - ツールテンプレート  
- [[Projects/_TEMPLATES/Content-Project/00_PROJECT-OVERVIEW]] - コンテンツテンプレート

### 関連ドキュメント
- [[Want/README]] - アイデア管理システム
- [[PROJECT-WORKFLOW]] - プロジェクトワークフロー
- [[Development-Docs/README]] - 従来のドキュメント管理

## 🎊 このシステムの効果

### 開発効率向上
- プロジェクト関連ファイルの一元管理
- テンプレートによる高速プロジェクト開始
- 段階的な進行管理

### 品質向上
- プロジェクトタイプ別のベストプラクティス
- 包括的なドキュメント管理
- 継続的な進捗追跡

### 学習・成長促進
- プロジェクトの体系的な記録
- 振り返り・改善のためのデータ蓄積
- 知見の再利用・共有

---

**📞 サポート・質問**  
このシステムについて質問や改善提案があれば、お気軽にお声かけください。使いながら一緒により良いシステムに改善していきましょう！

**作成日**: 2025年6月18日  
**最終更新**: 2025年6月18日
