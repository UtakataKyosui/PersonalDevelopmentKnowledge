
---
# チャンクポモドーロ Slackワークフロー連携 — Cloudflare Workers プロジェクトコンテキスト

---

## 概要

- **目的**: 平日の11時〜19時（13時〜14時除く）に2時間チャンクを4回作成し、ポモドーロ（25分作業＋5分休憩）サイクルをSlackワークフローの通知で管理する。
    
- **環境**: Cloudflare Workers + Slackワークフロービルダー（Webhookトリガー利用）。
    
- **通知**: SlackワークフローWebhook URLへPOSTする形でチャンク開始、作業開始、休憩開始、チャンク終了の通知を送信。
    
- **ユーザー操作**: Slackワークフロー内で「目標入力」や「達成/未達成の選択」を行う。
    

---

## 要件詳細

### チャンク作成

- 平日の11:00、13:00〜14:00は除く13時台は休憩。
    
- 11:00〜13:00、15:00〜17:00、17:00〜19:00、19:00〜21:00にチャンク作成。
    
- 各チャンクはポモドーロタイマー（25分作業＋5分休憩）×4セット。
    

### Slack連携

- SlackワークフローのWebhook URLをCloudflare Workersから叩く形で通知。
    
- チャンク開始時に「目標入力フォーム」をSlackで提示。
    
- ポモドーロ作業開始・休憩開始時に通知（必要に応じてCloudflare Workers側で送信可能）。
    
- チャンク終了時に「目標達成／未達成」の確認フォームをSlackワークフローで表示。
    
- 1日の終わりに達成状況のまとめ通知。
    
- 翌日朝に未達成タスクの提案通知。
    

---

## 技術的ポイント

- **Cloudflare Workers**
    
    - `scheduled` イベントで定期実行（Cronトリガー）設定。
        
    - `.dev.vars` ファイルで環境変数管理（SlackワークフローWebhook URL）。
        
    - 本番環境では `wrangler secret put` によるSecret管理。
        
    - SlackワークフローWebhook URLに対しJSON payloadをPOSTするだけのシンプル構成。
        
- **Slackワークフロー**
    
    - GUIでワークフロー作成しWebhook URL取得。
        
    - フォーム・選択肢の入力ステップで目標設定や達成状況を受け付ける。
        
    - Bot認証不要で、Slackワークスペース内のユーザー操作のみ。
        

---

## 現状の成果物

- Cloudflare Workers のCron設定サンプルと通知送信コードスケルトン（TypeScript）。
    
- Slack通知テンプレート例（チャンク開始、作業・休憩開始、終了振り返り、1日まとめ、翌日提案）。
    
- 環境変数管理の運用方針（`.dev.vars` + `wrangler secret`）と `package.json` スクリプト例。
    

---

## 今後の拡張案

- KVストアを使ったチャンクの状態管理（目標、達成/未達成、コメント保存）。
    
- Slackワークフロー以外にBotトークンを用いたより高度な双方向インタラクション。
    
- ポモドーロタイマーの細かなリアルタイム通知（25分／5分の自動送信）。
    
- タスクの未達成履歴を分析して、翌日の提案を自動化。
    

---

## 参照情報

- wrangler公式ドキュメント: [https://developers.cloudflare.com/workers/cli-wrangler/](https://developers.cloudflare.com/workers/cli-wrangler/)
    
- SlackワークフローWebhook: [https://api.slack.com/workflows/webhooks](https://api.slack.com/workflows/webhooks)
    
- Cloudflare WorkersでのCronトリガー: [https://developers.cloudflare.com/workers/runtime-apis/cron-triggers](https://developers.cloudflare.com/workers/runtime-apis/cron-triggers)
    

---

このコンテキストをもとに、今後の開発支援や具体的なコード作成・レビューをお願いします。  
必要に応じて詳細説明や追加情報も提供可能です。
