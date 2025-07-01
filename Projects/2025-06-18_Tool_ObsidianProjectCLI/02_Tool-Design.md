### 02_Tool-Design.md - CLI設計詳細

# CLI ツール設計書

## コマンド体系

### 基本構造
```
opj [global-options] <command> [command-options] [arguments]
```

### グローバルオプション
- `-v, --verbose` - 詳細出力
- `-q, --quiet` - エラーのみ出力
- `-c, --config FILE` - 設定ファイル指定
- `--obsidian-path DIR` - Obsidianパス指定
- `--dry-run` - 実行確認のみ
- `--no-color` - カラー出力無効

## コマンド詳細仕様

### new コマンド
新規プロジェクトの作成

**基本形式**:
```bash
opj new [NAME] [OPTIONS]
```

**オプション**:
- `-t, --type TYPE` - プロジェクトタイプ
- `-p, --priority LEVEL` - 優先度
- `-d, --description TEXT` - 説明
- `--template NAME` - テンプレート指定
- `--no-git` - Git初期化をスキップ
- `-i, --interactive` - インタラクティブモード

**使用例**:
```bash
opj new "My Project" --type web-app --priority high
opj new --interactive
```

### list コマンド
プロジェクト一覧の表示

**基本形式**:
```bash
opj list [OPTIONS]
```

**オプション**:
- `-s, --status STATUS` - ステータスフィルタ
- `-t, --type TYPE` - タイプフィルタ
- `-p, --priority LEVEL` - 優先度フィルタ
- `--sort FIELD` - ソート項目
- `-f, --format FORMAT` - 出力形式

**使用例**:
```bash
opj list --status development
opj list --type web-app --format json
```

### status コマンド
プロジェクトステータス管理

**基本形式**:
```bash
opj status PROJECT [OPTIONS]
```

**オプション**:
- `--set STATUS` - ステータス設定
- `-n, --note TEXT` - 変更ノート
- `-l, --log` - 履歴表示

**使用例**:
```bash
opj status "My Project" --set development --note "Started coding"
opj status "My Project" --log
```

## エラーハンドリング

### エラーレベル
1. **Critical Error**: 設定ファイル読み込み失敗など
2. **Command Error**: コマンド実行エラー
3. **Validation Error**: 入力値検証エラー
4. **Warning**: 非致命的な問題

### エラーメッセージ例
```bash
❌ Error: Project 'Unknown Project' not found

Suggestions:
  • Check project name spelling
  • Use 'opj list' to see available projects
  • Create new project with 'opj new "Unknown Project"'
```

## ユーザビリティ設計

### プログレス表示
```bash
🚀 Creating project "My App"...
📁 Creating directory... ████████████████████████████████ 100%
📄 Copying template...  ████████████████████████████████ 100%
✏️  Processing files...  ████████████████████████████████ 100%
✅ Project created successfully!
```

### カラーコード
- **成功**: 緑色 (✅)
- **エラー**: 赤色 (❌)
- **警告**: 黄色 (⚠️)
- **情報**: 青色 (ℹ️)
- **進行中**: シアン (🚧)

## パフォーマンス要件

### レスポンス時間
- `opj list`: < 200ms (100プロジェクト以下)
- `opj new`: < 3秒 (テンプレートコピー含む)
- `opj status`: < 100ms
- `opj info`: < 500ms

### メモリ使用量
- 基本動作: < 50MB
- 大量プロジェクト(1000+): < 200MB

## 設定管理

### 設定ファイル場所
- Linux/macOS: `~/.config/opj/config.yaml`
- Windows: `%APPDATA%/opj/config.yaml`

### 設定項目
```yaml
obsidian_path: "/path/to/obsidian"
projects_dir: "Projects"
templates_dir: "Projects/_TEMPLATES"
default_type: "web-app"
default_priority: "medium"
auto_git_init: true
editor: "code"
```

## 今後の拡張計画

### v0.2.0
- テンプレート管理機能
- プロジェクト移行機能
- 設定管理UI

### v0.3.0
- チーム機能
- プロジェクト共有
- Web UI

### v1.0.0
- プラグインシステム
- カスタムワークフロー
- 高度な分析機能
