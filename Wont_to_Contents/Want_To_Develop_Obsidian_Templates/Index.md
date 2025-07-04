
# 📝 QuickDraft - Obsidian 拡張機能 概要

**QuickDraft** は、アイデア・要件・仕様などをすばやく記録するための Obsidian 向け拡張機能です。  
ユーザー定義のテンプレートを活用して、瞬間的な発想や設計を構造的に残せます。

---

## 🎯 特徴

- 🚀 **瞬時に記録**：ホットキーやコマンドパレットから即座に入力モーダルを起動  
- 🧩 **テンプレート選択**：目的に応じた記録フォーマットを簡単に適用  
- ✍️ **ユーザー定義テンプレート対応**：自由にテンプレートを作成・管理可能  
- 🔄 **プレースホルダ展開**：`{{title}}` や `{{date}}` などで自動入力  
- 📁 **Markdown保存**：記録はすべて Obsidian Vault 内に Markdown ファイルとして残せる

---

## 📂 構成モジュール（抜粋）

| モジュール | 役割 |
|------------|------|
| `main.ts` | プラグイン初期化・コマンド登録 |
| `template-manager.ts` | テンプレートの読み書き処理 |
| `placeholder.ts` | プレースホルダの置換ロジック |
| `quickdraft-modal.ts` | クイック記録用の入力UI |
| `settings-tab.ts` | テンプレート管理画面（設定UI） |

---

## 📌 用途例

- アイデアスケッチの即時記録  
- 要件定義や設計メモの構造化入力  
- バグ報告や改善提案のテンプレート化  
- 日次ログやふりかえりテンプレートなどのルーチン記録

---

## 🛠 今後の予定（抜粋）

- カスタムテンプレートの並び順や分類管理  
- 音声入力・Webクリップとの連携  
- 外部サービス（GitHub Issueなど）との同期機能
