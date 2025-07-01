`Claude Code`による`Vibe Coding`の効率化において、`Git WorkTree`の管理を効率化するCLIツールが`Phantom`。
`tmux`か`fzf`との統合機能も備えている。

## デモ動画のコマンド確認
```sh 
phantom create issue-111 --tmux-h

phantom create issue-112 --tmux-v

phantom create issue-91 --shell
```


> [!NOTE] 使うために必須なツール
> これ見ると、`tmux`は必須な気がする

## 設定ファイル `phantom.config.json`

```json
{
  "postCreate": {
    "copyFiles": [
      ".env",
      ".env.local",
      "config/local.json"
    ],
    "commands": [
      "pnpm install",
      "pnpm build"
    ]
  }
}
```