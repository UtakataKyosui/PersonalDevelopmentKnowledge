# tmuxによるClaude Code Company管理システム

> **原文**: [Claude Codeを並列組織化してClaude Code "Company"にするtmuxコマンド集](https://zenn.dev/kazuph/articles/beb87d102bd4f5)

## 概要

tmuxの複数paneでClaude Codeインスタンスを並列実行し、効率的にタスクを分散処理する「Claude Code Company」システム。部長と部下のような役割分担で複数のClaudeを協調動作させる手法。

## 基本セットアップ

### 前提条件
```bash
# エイリアス設定
alias cc="claude"

# 危険なオプション（自己責任）
claude --dangerously-skip-permissions
```

### tmux pane構成作成
```bash
# 5つのpaneに分割
tmux split-window -h && tmux split-window -v && tmux select-pane -t 0 && tmux split-window -v && tmux select-pane -t 2 && tmux split-window -v && tmux select-pane -t 4 && tmux split-window -v
```

### pane番号確認
```bash
tmux list-panes -F "#{pane_index}: #{pane_id} #{pane_current_command} #{pane_active}"
```

出力例：
```
0: %22 zsh 1 (メインpane)
1: %27 zsh 0 (部下1)
2: %28 zsh 0 (部下2)
3: %25 zsh 0 (部下3)
4: %29 zsh 0 (部下4)
5: %26 zsh 0 (部下5)
```

## Claude Codeセッション管理

### 並列起動
```bash
# 全paneで並列起動（実際のpane IDに置き換え）
tmux send-keys -t %27 "cc" && sleep 0.1 && tmux send-keys -t %27 Enter & \
tmux send-keys -t %28 "cc" && sleep 0.1 && tmux send-keys -t %28 Enter & \
tmux send-keys -t %25 "cc" && sleep 0.1 && tmux send-keys -t %25 Enter & \
tmux send-keys -t %29 "cc" && sleep 0.1 && tmux send-keys -t %29 Enter & \
tmux send-keys -t %26 "cc" && sleep 0.1 && tmux send-keys -t %26 Enter & \
wait
```

### タスク割り当て

#### 基本テンプレート
```bash
tmux send-keys -t %27 "cd 'ワーキングディレクトリ' && あなたはpane1です。タスク内容。エラー時は[pane1]でtmux send-keys -t %22でメイン報告。" && sleep 0.1 && tmux send-keys -t %27 Enter
```

#### 並列タスク割り当て
```bash
tmux send-keys -t %27 "タスク1の内容" && sleep 0.1 && tmux send-keys -t %27 Enter & \
tmux send-keys -t %28 "タスク2の内容" && sleep 0.1 && tmux send-keys -t %28 Enter & \
tmux send-keys -t %25 "タスク3の内容" && sleep 0.1 && tmux send-keys -t %25 Enter & \
wait
```

## 報連相システム

### 部下からメインへの報告
```bash
tmux send-keys -t %22 '[pane番号] 報告内容' && sleep 0.1 && tmux send-keys -t %22 Enter
```

### 報告例
```bash
# 完了報告
tmux send-keys -t %22 '[pane1] タスク完了しました' && sleep 0.1 && tmux send-keys -t %22 Enter

# エラー報告
tmux send-keys -t %22 '[pane3] エラーが発生しました：詳細内容' && sleep 0.1 && tmux send-keys -t %22 Enter
```

## トークン管理

### /clearコマンド実行

#### 実行タイミング
- タスク完了時（新しいタスクに集中させるため）
- トークン使用量が高くなった時
- エラーが頻発している時
- 複雑な作業から単純な作業に切り替える時

#### 個別クリア
```bash
tmux send-keys -t %27 "/clear" && sleep 0.1 && tmux send-keys -t %27 Enter
```

#### 並列クリア
```bash
tmux send-keys -t %27 "/clear" && sleep 0.1 && tmux send-keys -t %27 Enter & \
tmux send-keys -t %28 "/clear" && sleep 0.1 && tmux send-keys -t %28 Enter & \
tmux send-keys -t %25 "/clear" && sleep 0.1 && tmux send-keys -t %25 Enter & \
wait
```

## 状況確認コマンド

### 個別pane確認
```bash
# 各paneの最新状況確認
tmux capture-pane -t %27 -p | tail -10
tmux capture-pane -t %28 -p | tail -10
```

### 全pane一括確認
```bash
for pane in %27 %28 %25 %29 %26; do
  echo "=== $pane ==="
  tmux capture-pane -t $pane -p | tail -5
done
```

## ベストプラクティス

### 1. 明確な役割分担
- pane番号を必ず伝える
- 担当タスクを具体的に指示
- エラー時の報告方法を明記

### 2. 効率的なコミュニケーション
- ワンライナー形式での報告徹底
- [pane番号]プレフィックス必須
- 具体的なエラー内容の報告

### 3. トークン使用量管理
- 定期的な/clear実行
- 大量トークン消費の監視
- ccusageでの使用量確認

### 4. エラー対処
- Web検索による解決策調査を指示
- 具体的エラー内容の共有
- 成功事例の横展開

## 活用例

### 大規模タスクの分散処理
- **資料作成**: 各paneで異なる章を担当
- **エラー解決**: 各paneで異なる角度から調査
- **知見共有**: 成功事例の文書化と横展開
- **品質管理**: 並列でのファイル修正と確認

## 注意事項

- 部下は直接/clearできない（tmux経由でのみ可能）
- 報告は必ずワンライナー形式で
- pane番号の確認を怠らない
- トークン使用量の定期確認
- 複雑な指示は段階的に分割

## まとめ

tmuxとClaude Codeを組み合わせることで、複数のAIエージェントを効率的に管理し、大規模タスクの並列処理が可能。適切な報連相システムとトークン管理により、チーム開発のような協調作業を実現。