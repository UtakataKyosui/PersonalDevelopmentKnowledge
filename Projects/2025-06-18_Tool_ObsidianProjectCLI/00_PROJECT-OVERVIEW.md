---
title: "obsidian-project-cli"
type: "Tool/CLI"
category: "Tool"
status: "開発中"
priority: "high"
created: "2025-06-18"
updated: "2025-06-18"
tags:
  - project
  - tool
  - cli
  - rust
  - obsidian
  - project-management
  - priority-high
---

# obsidian-project-cli

## 📋 プロジェクト基本情報

| 項目 | 内容 |
|------|------|
| **ツール名** | obsidian-project-cli (opj) |
| **カテゴリ** | CLI Tool |
| **ステータス** | 開発中 |
| **優先度** | high |
| **開始日** | 2025-06-18 |
| **予定期間** | 2週間 |
| **メイン言語** | Rust |

## 🎯 ツール概要

### 何を作るのか
Obsidianプロジェクト管理システムを円滑に運用するためのCLIツール。プロジェクトの作成、テンプレート適用、ステータス管理、移行作業などを自動化し、開発者の生産性を向上させる。

### なぜ作るのか（動機・背景）
- **手動作業の削減**: プロジェクト作成時のテンプレートコピー・リネーム作業を自動化
- **一貫性の保証**: 命名規則やディレクトリ構造の統一
- **効率的な管理**: プロジェクトステータスの一括管理・更新
- **移行支援**: 既存システムからの段階的移行をサポート

### 誰が使うのか（ターゲットユーザー）
- **主要ユーザー**: Obsidianでプロジェクト管理を行う開発者・クリエイター
- **使用場面**: 新規プロジェクト開始、プロジェクト管理、進捗確認
- **技術レベル**: CLIツールを使い慣れた中級者以上

## 🛠️ ツールタイプ

- [x] **CLI ツール** - コマンドライン操作
- [ ] **ライブラリ・パッケージ** - 他のプロジェクトで使用
- [ ] **VS Code拡張** - エディタ機能拡張
- [ ] **ブラウザ拡張** - ブラウザ機能拡張
- [ ] **デスクトップアプリ** - スタンドアロンアプリ
- [x] **開発ツール** - 開発支援ツール

## ⚙️ 主要機能

### 核となる機能（MVP）
1. **プロジェクト作成 (`opj new`)**
   - **概要**: 新規プロジェクトの作成とテンプレート適用
   - **入力**: プロジェクト名、タイプ、説明
   - **処理**: ディレクトリ作成、テンプレートコピー、プレースホルダー置換
   - **出力**: 完全にセットアップされたプロジェクトディレクトリ
   - **使用例**: `opj new "タスク管理ツール" --type web-app --priority high`

2. **プロジェクト一覧 (`opj list`)**
   - **概要**: 既存プロジェクトの一覧表示とフィルタリング
   - **入力**: フィルタ条件（ステータス、タイプ、優先度）
   - **処理**: プロジェクトディレクトリスキャン、メタデータ解析
   - **出力**: フォーマットされたプロジェクト一覧
   - **使用例**: `opj list --status development --type web-app`

3. **ステータス更新 (`opj status`)**
   - **概要**: プロジェクトステータスの更新と履歴記録
   - **入力**: プロジェクト名/ID、新ステータス、コメント
   - **処理**: メタデータ更新、履歴追加、ログ記録
   - **出力**: 更新確認メッセージ
   - **使用例**: `opj status "タスク管理ツール" --set development --note "基本機能実装開始"`

### 追加機能
- [ ] **テンプレート管理** - カスタムテンプレートの追加・編集
- [ ] **プロジェクト移行** - 既存プロジェクトの新システムへの移行
- [ ] **進捗レポート** - プロジェクト進捗の可視化・レポート生成
- [ ] **設定管理** - ユーザー設定・環境設定の管理
- [ ] **バックアップ** - プロジェクトデータのバックアップ・復元
- [ ] **統計表示** - プロジェクト統計・分析データの表示

## 🔧 技術スタック

### 開発言語・フレームワーク
- **メイン言語**: Rust 1.70+
- **CLI フレームワーク**: clap 4.0 (コマンドライン引数解析)
- **非同期処理**: tokio (ファイルI/O最適化)
- **シリアライゼーション**: serde + serde_yaml (設定・メタデータ管理)

### 依存クレート
```toml
[dependencies]
clap = { version = "4.0", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
serde_yaml = "0.9"
tokio = { version = "1.0", features = ["full"] }
chrono = { version = "0.4", features = ["serde"] }
anyhow = "1.0"
colored = "2.0"
indicatif = "0.17"
dialoguer = "0.10"
walkdir = "2.0"
regex = "1.0"
```

### 配布・パッケージング
- **パッケージレジストリ**: crates.io
- **バイナリ配布**: GitHub Releases
- **インストール方法**: Homebrew/Chocolatey (将来的)

## 📟 CLI設計

### コマンド構造
```bash
opj [global-options] <command> [command-options] [arguments]
```

### 主要コマンド
```bash
# プロジェクト管理
opj new <name>              # 新規プロジェクト作成
opj list                    # プロジェクト一覧表示
opj status <project>        # プロジェクトステータス管理
opj info <project>          # プロジェクト詳細情報表示

# テンプレート管理
opj template list           # テンプレート一覧
opj template add <path>     # テンプレート追加
opj template edit <name>    # テンプレート編集

# ユーティリティ
opj migrate <source>        # 既存プロジェクト移行
opj config                  # 設定管理
opj backup                  # バックアップ作成
opj stats                   # 統計情報表示

# ヘルプ・バージョン
opj help                    # ヘルプ表示
opj version                 # バージョン表示
```

### オプション・フラグ設計
```bash
Global Options:
  -v, --verbose       詳細な出力
  -q, --quiet         静寂モード（エラーのみ出力）
  -c, --config FILE   設定ファイル指定
  --obsidian-path DIR Obsidianディレクトリパス指定
  --dry-run           実行せずに確認のみ
  --no-color          カラー出力無効

New Command Options:
  -t, --type TYPE         プロジェクトタイプ (web-app|tool|content|api)
  -p, --priority LEVEL    優先度 (low|medium|high)
  -d, --description TEXT  プロジェクト説明
  --template NAME         使用テンプレート指定
  --no-git               Git初期化をスキップ

List Command Options:
  -s, --status STATUS     ステータスでフィルタ
  -t, --type TYPE         タイプでフィルタ
  -p, --priority LEVEL    優先度でフィルタ
  --sort FIELD            ソート項目 (name|date|status|priority)
  --format FORMAT         出力形式 (table|json|yaml)

Status Command Options:
  --set STATUS            新しいステータスを設定
  --note TEXT             ステータス変更の注記
  --log                   ステータス履歴を表示
```

## 🗂️ ファイル・データ構造

### プロジェクトメタデータ
```yaml
# .project-meta.yaml
name: "タスク管理ツール"
type: "web-app"
status: "development"
priority: "high"
created: "2025-06-18T10:00:00Z"
updated: "2025-06-18T15:30:00Z"
description: "React + TypeScriptによるモダンなタスク管理アプリ"
tags:
  - project
  - web-app
  - react
  - typescript
  - priority-high
technologies:
  - React
  - TypeScript
  - Tailwind CSS
  - Prisma
status_history:
  - status: "idea"
    timestamp: "2025-06-18T10:00:00Z"
    note: "プロジェクト作成"
  - status: "planning"
    timestamp: "2025-06-18T12:00:00Z"
    note: "要件定義開始"
  - status: "development"
    timestamp: "2025-06-18T15:30:00Z"
    note: "基本機能実装開始"
```

### 設定ファイル
```yaml
# ~/.config/opj/config.yaml
obsidian_path: "/Users/username/Documents/Obsidian-Vault"
projects_dir: "Projects"
templates_dir: "Projects/_TEMPLATES"
default_type: "web-app"
default_priority: "medium"
auto_git_init: true
editor: "code"
date_format: "%Y-%m-%d"
status_values:
  - "idea"
  - "planning"
  - "development"
  - "testing"
  - "completed"
  - "archived"
  - "cancelled"
```

## 💻 実装アーキテクチャ

### モジュール構成
```rust
src/
├── main.rs                 # エントリーポイント
├── cli/
│   ├── mod.rs             # CLI定義
│   ├── commands/
│   │   ├── mod.rs
│   │   ├── new.rs         # プロジェクト作成
│   │   ├── list.rs        # プロジェクト一覧
│   │   ├── status.rs      # ステータス管理
│   │   ├── template.rs    # テンプレート管理
│   │   └── migrate.rs     # 移行コマンド
│   └── args.rs            # 引数定義
├── core/
│   ├── mod.rs
│   ├── project.rs         # プロジェクト構造体
│   ├── template.rs        # テンプレート処理
│   ├── metadata.rs        # メタデータ管理
│   └── status.rs          # ステータス管理
├── fs/
│   ├── mod.rs
│   ├── operations.rs      # ファイルシステム操作
│   └── scanner.rs         # ディレクトリスキャン
├── config/
│   ├── mod.rs
│   └── settings.rs        # 設定管理
└── utils/
    ├── mod.rs
    ├── date.rs            # 日付処理
    ├── template.rs        # テンプレート置換
    └── output.rs          # 出力フォーマット
```

### 主要な型定義
```rust
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Project {
    pub name: String,
    pub project_type: ProjectType,
    pub status: ProjectStatus,
    pub priority: Priority,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub technologies: Vec<String>,
    pub status_history: Vec<StatusEntry>,
    pub path: PathBuf,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ProjectType {
    WebApp,
    Tool,
    Content,
    Api,
    Custom(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ProjectStatus {
    Idea,
    Planning,
    Development,
    Testing,
    Completed,
    Archived,
    Cancelled,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StatusEntry {
    pub status: ProjectStatus,
    pub timestamp: DateTime<Utc>,
    pub note: Option<String>,
}
```

## 🎨 ユーザー体験設計

### インタラクティブプロジェクト作成
```bash
$ opj new
✨ Create a new project

? Project name: タスク管理ツール
? Project type: 
  > Web Application
    CLI Tool
    Content/Article
    API/Backend
? Priority: 
    Low
  > Medium
    High
? Description (optional): React + TypeScriptによるモダンなタスク管理アプリ

🚀 Creating project "タスク管理ツール"...
📁 Creating directory: Projects/2025-06-18_Web-App_タスク管理ツール/
📄 Copying template: Web-App-Project -> タスク管理ツール
✏️  Updating project metadata...
📝 Creating initial files...

✅ Project "タスク管理ツール" created successfully!

📍 Location: Projects/2025-06-18_Web-App_タスク管理ツール/
📋 Next steps:
  1. Review project overview: opj info "タスク管理ツール"
  2. Edit requirements: edit 01_Requirements.md
  3. Start development: opj status "タスク管理ツール" --set development
```

### 美しいプロジェクト一覧
```bash
$ opj list --status development

📊 Active Projects (Development)

┌─────────────────────────────┬──────────┬──────────┬────────────┬─────────────┐
│ Name                        │ Type     │ Priority │ Status     │ Updated     │
├─────────────────────────────┼──────────┼──────────┼────────────┼─────────────┤
│ 🌐 タスク管理ツール         │ Web-App  │ High     │ Development│ 2 days ago  │
│ 🛠️  CLI-Generator           │ Tool     │ Medium   │ Development│ 1 week ago  │
│ 📝 TypeScript実践ガイド     │ Content  │ High     │ Development│ 3 days ago  │
└─────────────────────────────┴──────────┴──────────┴────────────┴─────────────┘

💡 Use 'opj info <name>' for detailed information
🔄 Use 'opj status <name>' to update status
```

### 進捗追跡とステータス更新
```bash
$ opj status "タスク管理ツール" --set testing --note "基本機能実装完了、テスト開始"

🔄 Updating project status...

Project: タスク管理ツール
Status: Development → Testing
Note: 基本機能実装完了、テスト開始

✅ Status updated successfully!

📈 Progress:
  ✅ Idea (2025-06-18)
  ✅ Planning (2025-06-18)
  ✅ Development (2025-06-20)
  🚧 Testing (2025-06-25) ← Current
  ⏳ Completed
  ⏳ Archived

💡 Next: opj status "タスク管理ツール" --set completed when ready
```

## 🧪 テスト戦略

### テストレベル
- **単体テスト**: コア機能（プロジェクト作成、メタデータ管理）のテスト
- **統合テスト**: CLI コマンド全体の動作テスト
- **E2Eテスト**: 実際のプロジェクトライフサイクル全体のテスト
- **パフォーマンステスト**: 大量プロジェクトでの動作確認

### テスト実装
```bash
# テスト実行
cargo test                  # 全テスト
cargo test --lib           # 単体テスト
cargo test --test cli      # CLI統合テスト
cargo test --release       # リリースモードテスト

# カバレッジ
cargo tarpaulin --out Html
```

## 🚀 開発・リリース計画

### フェーズ1: 基盤構築（期間：1週間）
- [ ] プロジェクト環境構築（Cargo.toml、CI/CD）
- [ ] 基本的なCLI構造実装（clap設定）
- [ ] コア機能実装（Project, Template構造体）
- [ ] 基本的なファイルシステム操作
- [ ] 設定ファイル管理

### フェーズ2: 主要機能実装（期間：1週間）
- [ ] `opj new` コマンド実装
- [ ] `opj list` コマンド実装
- [ ] `opj status` コマンド実装
- [ ] テンプレート処理システム
- [ ] メタデータ管理システム

### フェーズ3: 品質向上・仕上げ（期間：数日）
- [ ] エラーハンドリング強化
- [ ] ユーザビリティ改善（プログレスバー、色分け）
- [ ] テストカバレッジ向上
- [ ] ドキュメント作成
- [ ] パフォーマンス最適化

### フェーズ4: リリース準備（期間：数日）
- [ ] パッケージング（バイナリ作成）
- [ ] CI/CD整備（GitHub Actions）
- [ ] crates.io公開準備
- [ ] README・使用例整備
- [ ] 初回リリース（v0.1.0）

## 📈 成功指標・目標

### 技術目標
- [ ] 基本機能（new, list, status）の完全動作
- [ ] テストカバレッジ80%以上
- [ ] プロジェクト作成時間を手動の1/10に短縮
- [ ] エラー率1%未満

### 利用目標
- [ ] 自分自身での日常的な使用
- [ ] GitHub Star 50以上
- [ ] crates.io ダウンロード 100/月
- [ ] コミュニティフィードバック収集

### 学習目標
- [ ] Rust CLIアプリケーション開発の習得
- [ ] clap, serde, tokio等の実践的使用
- [ ] ファイルシステム操作とメタデータ管理
- [ ] ユーザビリティを重視したCLI設計

## ⚠️ リスク・課題

### 技術的リスク
- **ファイルシステム操作の複雑さ**: 異なるOS間での互換性問題
  - 対策: 十分なテスト、pathライブラリの活用
- **テンプレート処理の複雑化**: 複雑な置換ロジックの実装困難
  - 対策: シンプルな置換から開始、段階的に拡張

### ユーザビリティリスク
- **学習コストの高さ**: CLIコマンドの覚えにくさ
  - 対策: インタラクティブモード提供、充実したヘルプ
- **既存ワークフローとの摩擦**: 手動操作からの移行コスト
  - 対策: 段階的導入サポート、既存プロジェクト移行機能

## 💡 アイデア・メモ

### 将来的な拡張アイデア
- Web UI版の開発（wasm-bindgen使用）
- VS Code拡張としての統合
- Obsidianプラグインとしての提供
- チーム共有機能（Git連携）
- プロジェクトテンプレートのマーケットプレイス

### 参考にするツール
- **[cargo-generate](https://github.com/cargo-generate/cargo-generate)**: テンプレートからのプロジェクト生成
- **[git-cliff](https://github.com/orhun/git-cliff)**: 美しいCLI出力デザイン
- **[starship](https://github.com/starship/starship)**: 高速で美しいプロンプト

### 技術学習リソース
- **[The Rust Programming Language](https://doc.rust-lang.org/book/)**: Rust基礎
- **[Command Line Rust](https://www.oreilly.com/library/view/command-line-rust/9781098109424/)**: Rust CLI開発
- **[clap Documentation](https://docs.rs/clap/latest/clap/)**: CLI引数解析

## 🔗 関連ドキュメント

- **詳細設計**: [[02_Tool-Design]]
- **開発ログ**: [[03_Development-Log]]
- **パッケージ戦略**: [[04_Package-Strategy]]
- **使用例集**: [[examples/]]

## 📈 ステータス更新履歴

### 2025-06-18
- **ステータス**: アイデア → 計画中
- **実施内容**: ツール概要・設計の詳細化
- **次のアクション**: Cargo プロジェクト初期化、基本構造実装

### 2025-06-18 (予定)
- **ステータス**: 計画中 → 開発中
- **実施内容**: 開発環境構築完了
- **次のアクション**: core モジュール実装開始

---

**作成日**: 2025-06-18  
**最終更新**: 2025-06-18  
**プロジェクトフォルダ**: `Projects/2025-06-18_Tool_ObsidianProjectCLI/`
