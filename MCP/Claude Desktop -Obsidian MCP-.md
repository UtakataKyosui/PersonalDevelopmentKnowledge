## 必要なもの
- `npx`：`Node.js`が入っていればなんとかなる

### インストールコマンド
```bash
npx @smithery/cli install mcp-obsidian --client claude
```

```json
"mcp-obsidian": {
  "command": "npx",
  "args": [
	"-y",
	"@smithery/cli@latest",
	"run",
	"mcp-obsidian",
	"--config",
	"\"{\\\"vaultPath\\\":\\\"＜Obsidian Vaultのパス＞\\\"}\""
  ]
}
```

https://dev.classmethod.jp/articles/obsidian-mcp-claude-desktop-integration-hands-on/