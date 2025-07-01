# ccmanager - Git Worktree一括管理CLI

> **原文**: [ccmanager: Git Worktreeで並列に動くClaude Codeを一括管理する](https://zenn.dev/kbwok/articles/33fad69555d005)

## 概要

複数のClaude Codeセッションを一つのプロセスから一括管理できるCLIツール。Git Worktreeとの統合により、並行開発ワークフローを効率化。tmuxの代替ソリューションとして開発。

## 背景・課題

### Claude Code並行運用の課題
- Git Worktreeディレクトリの管理が大変
- tmuxを使った管理の欠点：
  - 既存のtmuxワークフローを変えたくない
  - どこで何をしていたか記憶できない
  - 視覚情報がマルチタスク的で認知負荷が高い
  - 複数worktreeの管理自体が面倒

## ccmanagerの特徴

### インストール・実行
```bash
# npmでインストール
npm install -g ccmanager

# 実行
ccmanager
```

### 主要機能
- **New Worktree**: ディレクトリとブランチを指定してワークツリー作成
- **Merge Worktree**: マージ元とマージ先のブランチを指定してmerge/rebase
- **Delete Worktree**: ワークツリーとブランチを削除
- **Configure Shortcuts**: メニューへ戻るキーの設定（デフォルト: Ctrl+E）
- **Exit**: ccmanager終了

### 戻るショートカット
- **Ctrl+E**: Claude Codeセッションからメニューに戻る
- **Configure Shortcuts**から変更可能

## セッションステータス管理

### 4つの状態表示
1. **ステータス表記なし**: Claude Codeをまだ実行していない状態
2. **Idle**: Claude Codeが何も作業せず、ユーザーの指示を待っている状態  
3. **Waiting**: ユーザーにYes/Noなどの追加インタラクションを求めている状態
4. **Busy**: Claude Codeがなんらかの作業をしている状態

### ステータス管理の利点
- どのセッションが入力を待っているか一目で分かる
- どのセッションが作業中か把握できる
- 効率的なセッション間切り替えが可能
- 複数タスクの並行進行を最適化

## ccmanagerのメリット

### tmuxワークフローとの共存
- 独立したプロセスとして動作
- 既存のtmuxの設定やワークフローに影響なし
- 従来の開発環境を維持したまま並行開発が可能

### 認知負荷の軽減
- 全ワークツリーを一つのメニューで管理
- ステータスの一目確認
- 複数ターミナルウィンドウやtmuxペインの管理不要

### Git Worktreeの統合
- Git Worktreeの作成、削除、マージをUIから直接実行
- コマンドラインを行き来する必要なし
- ワークツリー操作の一元化

## 使用例

### 基本的な並行開発フロー
1. **ccmanager**を起動
2. **New Worktree**で新しい機能ブランチのワークツリー作成
3. 任意のブランチを選択してClaude Codeセッション開始
4. **Ctrl+E**でメニューに戻り、別のワークツリーに切り替え
5. 複数のタスクを並行して進行
6. 作業完了後、**Merge Worktree**でマージ実行

### ステータス確認による効率化
- **Idle**状態のセッション：新しいタスクを投入
- **Waiting**状態のセッション：ユーザー入力が必要
- **Busy**状態のセッション：作業完了まで待機
- ステータス表記なし：Claude Codeセッション未開始

## 技術仕様

### 対応環境
- Node.js環境
- Git Worktree対応
- Claude Code CLI

### アーキテクチャ
- CLIベースのインタラクティブUI
- Git Worktreeとの直接統合
- リアルタイムステータス監視
- プロセス間通信によるセッション管理

## 導入効果

### 開発効率の向上
- 複数タスクの並行処理
- Git操作の簡略化
- セッション切り替えの高速化

### 管理負荷の軽減
- ワークツリーの可視化
- ステータスの一元管理
- コマンド実行の自動化

### ワークフローの改善
- 既存環境との共存
- 直感的な操作性
- スムーズな並行開発

## まとめ

ccmanagerは、Claude Codeを使った並行開発における課題を解決する実用的なツール。Git Worktreeとの統合により、複数のClaude Codeセッションを効率的に管理し、開発生産性を大幅に向上させる。tmuxに依存しない独立したソリューションとして、幅広い開発者に対応可能。

## リンク
- [GitHub Repository](https://github.com/kbwo/ccmanager)