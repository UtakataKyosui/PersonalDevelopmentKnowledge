# ✅ NextFlow 要件定義書（Rust版）

---

## 🔰 概要

NextFlowは、Claude Code の出力結果に基づいて自動的に次のプロンプトを生成し、継続的に処理を進行させるCLIツールである。  
JSONとMarkdown形式のTODO検出に対応し、セッション復元・tmux連携・ログ出力などを特徴とする。

---

## 📦 主な機能一覧

| #  | 機能カテゴリ        | 機能名                             | 内容                                                                 |
|----|---------------------|------------------------------------|----------------------------------------------------------------------|
| 1  | 基本ループ          | 自動プロンプトループ               | Claudeの出力から次のTODOを検出して自動投入                           |
| 2  | プロンプト強化      | Markdown / JSON TODO指定           | `## Next TODO` や `next_todos` JSON キーを指示テンプレートに明記       |
| 3  | JSONバリデーション | JSON SchemaでのTODO検証            | Claude応答がJSON形式の場合、スキーマに基づいて `next_todos` を検証   |
| 4  | ログ                | 出力ログ保存                       | `--log`指定時、入出力をMarkdownで `claude_output.md` に保存           |
| 5  | 環境変数設定        | `.env`読み込み                     | システムプロンプト・MCP設定・権限指定を `.env` から読み込み           |
| 6  | レート制限処理      | 自動待機・再開機能                 | Claudeがレート制限を返した場合、復帰時間を記録し `--continue` で再開 |
| 7  | セッション管理      | 状態保存・復元                     | セッションIDで作業状態を保存・復元可能                               |
| 8  | CLI制御             | `nxfw`コマンドで起動               | `nxfw "やること"` の形式で起動                                       |
| 9  | tmux連携表示        | Claude出力の分割表示               | `tmux`を使用し、制御画面とClaude出力を同時表示                        |
| 10 | TUI操作             | TUI UIで操作・状態表示            | `tui-rs`によりセッション操作や進捗確認などが可能                     |
| 11 | 起動演出            | ロゴ表示                           | ASCIIアートロゴ（`NEXTFLOW`）を起動時に表示（色付け対応）             |

---

## ⚙️ CLI仕様

```bash
nxfw "<やること>" [--log]
nxfw --continue
nxfw --session <SESSION_ID>
nxfw --list-sessions
````

---

## 📄 プロンプトテンプレート例

### ✅ Markdown形式

```
以下のタスクを実行してください：
{task}

出力は必ずMarkdown形式で、「## Next TODO」セクションにやるべきことを箇条書きで記述してください。
```

### ✅ JSON形式

```json
{
  "next_todos": [
    "ファイル分割の提案",
    "エンドポイントの設計"
  ]
}
```

---

## 🧪 JSON Schema バリデーション仕様

- Claudeの出力がJSON形式だった場合、以下の形式に従って `next_todos` を検証：
    

```json
{
  "type": "object",
  "properties": {
    "next_todos": {
      "type": "array",
      "items": { "type": "string" }
    }
  },
  "required": ["next_todos"]
}
```

- スキーマ検証に失敗した場合は警告を表示し、プロンプト投入を中止または確認プロンプトを挿入予定（設計検討中）
    

---

## 📁 ログ仕様

- ファイル名：`claude_output.md`
    
- 内容：プロンプトとClaudeの応答、および検出したNext TODOをMarkdownで保存
    

---

## 🖥️ tmux 連携

- `tmux`を起動し、左右にペインを分割
    
- 左ペイン：NextFlow管理TUI（進捗、ログ、制御）
    
- 右ペイン：Claude実行画面（標準出力・ログ確認）
    

---

## 🎨 ロゴ表示仕様

|項目|内容|
|---|---|
|表示内容|`NEXTFLOW` のASCIIアートロゴ|
|表示方式|`figlet-rs`, `ascii-art`, または静的埋め込み|
|色付け|`ansi_term`, `colored` による装飾対応|
|起動順序|画面クリア → ロゴ表示 → UI開始|

---

## 🧠 セッション管理仕様

|項目|内容|
|---|---|
|ID|UUIDまたは任意のセッション名|
|保存先|`session_<ID>.json`|
|再開|`nxfw --session <ID>` / `--continue`|
|一覧表示|`nxfw --list-sessions`|

---

## 🔜 拡張予定

- `--dry-run`モードで構成検証
    
- JSON/Markdown混在出力対応
    
- Git連携 / Slack通知 / GUI版
    
- Claude出力の構造整形（整形器）
    

---

## 🧰 技術スタック（想定）

|項目|候補ライブラリ|
|---|---|
|CLI|`clap`, `dotenvy`, `serde`|
|TUI|`tui-rs`, `ratatui`, `crossterm`|
|色表示|`colored`, `ansi_term`|
|JSON検証|`serde_json`, `schemars`（予定）|
|プロセス|`std::process::Command`|
|ログ|Markdown形式またはJSON並行保存|
