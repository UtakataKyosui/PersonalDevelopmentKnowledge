
# 🌙 PhantoMoon - Phantom Worktree UI Enhancer

**PhantoMoon** は [`phantom`](https://github.com/aku11i/phantom) の拡張実装です。  
Git worktree をより直感的かつ統合的に管理するため、TUI（Text User Interface）による可視化、GitHub連携、Issue/PR管理、そして開発効率の向上を目的としています。

## 🎯 目的

PhantoMoon の主な目的は以下の通りです：

- Git worktree の状態を一目で把握できる TUI を提供する
- worktree 名と GitHub Issue / PR を対応付け、プロジェクト全体の追跡を明確化
- TUI 上で PR 作成、ブランチ状態（存在・PR済・マージ済など）の確認を可能にする
- 開発ワークフローを GitHub Web UI に頼らずターミナル上で完結させる

## 🖥️ TUIレイアウト構成

以下は `ratatui` をベースとした画面イメージです：

```

+==============================================================================+  
| 📦 PhantoMoon - Git Worktree Manager for Humans |  
+------------------------------------------------------------------------------+  
| Issue # | Worktree Name | Branch | PR | Merged | Path |  
|---------+---------------+---------------+-------+--------+------------------|  
| 4321 | fix-login | fix/login | ✅PR | ❌ | ./worktrees/... |  
| 1234 | feature-ui | feat/ui | 🔄Open | ❌ | ./worktrees/... |  
| - | hotfix-build | hotfix/build | - | - | ./worktrees/... |  
+------------------------------------------------------------------------------+  
| ↑↓: 選択 Enter: Shell起動 c: Create d: Delete i: Issue紐付け p: PR作成 |  
+==============================================================================+  
| 💬 Status: Ready. |  
+==============================================================================+

````

### ✨ 主な画面構成

- **Worktreeテーブル**: 一覧表示＋カーソル移動で選択
- **キーバインド**:
  - `Enter`: Shell起動
  - `c`: worktree作成フォームを開く
  - `d`: 削除確認ダイアログ表示
  - `i`: Issue番号の手動紐付け
  - `p`: PR作成フォーム起動
- **ステータスバー**: 実行結果やエラーなどの表示

## 📦 技術スタック（依存関係）

| 用途                          | Crate / 技術名                                      | 補足                                                                 |
|-------------------------------|-----------------------------------------------------|----------------------------------------------------------------------|
| TUI描画                       | [`ratatui`](https://crates.io/crates/ratatui)       | 高機能なTUI描画ライブラリ。表・入力欄・ボックスUIなどを構築可能。  |
| ターミナル入出力制御         | [`crossterm`](https://crates.io/crates/crossterm)   | キーボード操作・端末制御・イベント処理などに対応。                  |
| Git操作（ローカル）          | [`git2`](https://crates.io/crates/git2)             | `libgit2` バインディング。ブランチ・Worktreeの確認等に利用。        |
| GitHub API操作（REST）       | [`octocrab`](https://crates.io/crates/octocrab)     | GitHub の Issue/PR 情報取得などに使用。REST および GraphQL 対応。 |
| GitHub CLI連携               | [`std::process::Command`] + [`gh`](https://cli.github.com/) | CLIからPR作成、PR状態確認、Web表示を実行可能。                     |
| 外部コマンド実行補助         | [`duct`](https://crates.io/crates/duct)（任意）     | `Command` のラップで直感的なシェルコマンド呼び出しが可能。         |
| 状態管理・遷移制御           | Rust構造体 + `enum` + State Pattern                 | TUIにおける画面状態の管理（List表示、フォーム入力、確認など）に使用。 |
| 設定ファイル保存（永続化）   | [`serde`](https://crates.io/crates/serde） + `serde_json` or `toml` | Worktreeごとのメタデータ（Issue IDなど）を保存。              |
| ログ出力・デバッグ           | [`tracing`](https://crates.io/crates/tracing) + `tracing-subscriber` | アクションログ、開発時のデバッグログなどに活用。                   |
| 非同期処理（必要に応じて）   | [`tokio`](https://crates.io/crates/tokio)           | GitHub APIが非同期での利用を必要とする場合に導入。                 |
| 入力フォーム補助（必要なら） | 自作 or [`tui-textarea`](https://crates.io/crates/tui-textarea) | PR作成フォームのタイトル・本文入力欄を作る場合に便利。              |

## 🔄 PR作成機能（`gh` CLI連携）

PhantoMoon では TUI から直接 PR を作成できます。以下は生成されるコマンド例：

```bash
gh pr create \
  --title "fix: login issue" \
  --body "Fixes #4321. Resolves a JS bug in login button." \
  --base main \
  --head fix/login
````

- **pキー**でPR作成フォームを開き、タイトル・本文・ベースブランチを入力
    
- 成功時には PR URL を TUI 上で表示
    
- Issueと紐付けされている場合は `Fixes #4321` を本文に自動挿入
    

## ✅ 前提条件

- `gh` CLI がインストール済みであること
    
- GitHub アカウントにログイン済みであること（`gh auth status`）
    
- 対象リポジトリが GitHub 上に存在していること
    
- Phantom によって worktree 管理が行われていること
    

## 🛠️ 今後の拡張案

- TUI内でPRレビューコメントの表示（`gh pr view --comments`）
    
- `git stash` や `git status` 表示で作業状態の可視化
    
- Worktree作成時にテンプレートから自動命名・Issue選択
    

## 📝 ライセンスと著作

- ベースツール：[phantom](https://github.com/aku11i/phantom)（MIT）
    
- PhantoMoonはこれを非公式に拡張する Rust製ユーティリティです
    