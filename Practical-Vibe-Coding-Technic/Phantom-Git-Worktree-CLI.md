# Phantom - Git Worktree管理CLI

> **原文**: [Claude Codeを20万円分使ってバイブコーディングの生産性を更に向上させるツールを作った](https://zenn.dev/aktriver/articles/2025-06-claude-code-200k-vibe-coding)

## 概要

Git worktreeの管理を効率化するCLIツール「Phantom」の開発記録。Claude Codeを完全バイブコーディングで活用し、約20万円（1600ドル）分のトークンを使用して1週間で完成。

## Phantomの特徴

### 基本コンセプト
```bash
phantom create feature-awesome
```

### ディレクトリ構造
```
your-project/
├── .git/
│   └── phantom/
│       └── worktrees/
│           ├── feature-awesome/ # worktree名（ブランチ名と同名）
│           ├── bugfix-login/
│           └── hotfix-critical/
```

### 主要機能
- **固定保存場所**: `.git/phantom/worktrees/`に統一
- **ブランチ名統一**: ワークツリー名 = ブランチ名
- **シェル補完**: fish, zsh対応
- **エディタ統合**: VSCode, Cursor対応
- **tmux/fzf統合**: `--tmux`, `--fzf`オプション

## 開発プロセス

### 開発期間・コスト
- **期間**: 6月1日〜6月8日（1週間）
- **コスト**: 約1600ドル（約23万円）※Claude Maxプランの定額$200
- **開発手法**: 完全バイブコーディング（コードを一切書かず）

### 段階的開発
1. **初日**: 基本機能実装（150ドル使用）
2. **平日**: 1日2時間の並行開発
3. **週末**: 集中開発（土曜3時間+8時間、日曜8時間）

### 並行開発フロー
- **3-5つのClaude Code**を同時起動
- **異なるGit worktree**で作業分散
- **思いついたことは即Issueに**

## Phantomの便利機能

### 自動セットアップ
```json
{
  "postCreate": {
    "copyFiles": [".env"],
    "commands": ["pnpm install"]
  }
}
```

### 実行例
```bash
# 基本操作
phantom create feature-payment
phantom list
phantom delete feature-payment

# 高度な操作
phantom create feature-awesome --copy-file .env
phantom exec feature-awesome "npm run build"

# エディタ統合
phantom create feature-ui --code  # VSCodeで開く
phantom create feature-api --cursor  # Cursorで開く

# tmux/fzf統合
phantom list --tmux
phantom switch --fzf
```

## バイブコーディングのベストプラクティス

### Issue管理
- **思いついたら即Issue化**: 「こういう機能欲しいな。Issueで提案して」
- **自動詳細化**: Claudeがコードベースから背景調査して高品質なIssueを作成
- **高速解決**: Issueが作られるそばから解決されるため蓄積しない

### Pull Request管理
- **Draft PR作成**: "good. create a draft pr"で自動作成
- **GitHub上でレビュー**: 視覚的に分かりやすい
- **直接修正指示**: 手元のClaude Codeに直接指示が効率的

### 並行タスク管理
- **明確な役割分担**: 各Claude Codeインスタンスに明確なタスク
- **独立性**: 異なるworktreeで干渉せず作業
- **集約レビュー**: GitHub上で全体を統一的に確認

## 技術スタック

### 実装言語・ツール
- **CLI**: Rust/Go（記事内で明示なし）
- **パッケージ管理**: Homebrew対応
- **シェル**: fish, zsh補完スクリプト自動生成
- **統合**: tmux, fzf, VSCode, Cursor

### インストール
```bash
brew install aku11i/tap/phantom
```

## 効果と学び

### 生産性向上
- **普段の何倍もの生産性**で開発進行
- **並行処理**: 複数のAIエージェントによる効率的なタスク分散
- **品質維持**: 自動化されたPR作成とレビュープロセス

### Claude Code活用ノウハウ
- **トークン管理**: 大量消費時の効率的な使い方
- **並行開発**: 複数インスタンスの効果的な管理方法
- **Issue駆動**: 思考の外部化による開発効率化

## まとめ

Phantomの開発を通じて、Claude Codeを使った大規模バイブコーディングの可能性を実証。Git worktreeという既存技術の課題を、AIとの協働により効率的に解決するツールを短期間で構築。Claude Maxプランの可能性を最大限活用した開発事例。