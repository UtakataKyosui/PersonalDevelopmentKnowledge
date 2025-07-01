# 📦 QuickDraft プラグイン 要件定義（実装構成編）

Obsidianにおいて、ユーザー定義テンプレートを用いたアイデア・要件記録支援ツール「QuickDraft」の実装に向けたファイル構成および各責務を整理する。

---

## 🗂 ディレクトリ構成（初期構築）
```

quickdraft/

├── main.ts                   // プラグイン本体（初期化・登録）

├── settings.ts               // 設定型とデフォルト定義

├── settings-tab.ts           // 設定画面（テンプレ管理UI）

├── template/

│   ├── template-manager.ts   // テンプレート読み書き処理

│   ├── placeholder.ts        // プレースホルダ変換処理

│   └── types.ts              // テンプレート型定義

├── ui/

│   ├── quickdraft-modal.ts   // 入力ポップアップUI

│   └── template-editor.ts    // テンプレート編集UI

├── utils/

│   └── path.ts               // パス操作・ユーティリティ

├── locales/

│   └── ja.json               // 日本語ローカライズ（任意）

├── styles/

│   └── style.css             // スタイル調整（任意）

├── manifest.json             // Obsidian Pluginメタ情報

└── README.md                 // プラグインの概要説明

````
---

## 📄 各ファイルの責務一覧

### `main.ts`
- プラグインのエントリーポイント
- `onload()` にて初期設定、コマンド登録、TemplateManager 初期化
- `unload()` にて後始末

### `settings.ts`
- ユーザー設定型（QuickDraftSettings）定義
- 保存先ディレクトリ・ファイル命名フォーマット・エディタ起動有無などの設定管理

### `settings-tab.ts`
- Obsidianの設定画面UI
- テンプレート一覧表示、編集・削除・新規作成
- 記録フォーマットやディレクトリ設定も含む

---

## 📁 `template/` モジュール群

### `template-manager.ts`
- テンプレートの読み込み、保存、削除処理
- `Map<string, string>` による一覧管理
- `.md` ファイルを読み書き対象とする

### `placeholder.ts`
- `{{title}}`, `{{date}}`, `{{time}}`, `{{cursor}}` などの文字列置換
- プレースホルダ→実際の値への変換ロジックを集中管理

### `types.ts`
- テンプレートファイルの構造や定義済みテンプレートの型を定義
- UIや保存処理との型整合を確保

---

## 🧩 `ui/` モジュール群

### `quickdraft-modal.ts`
- コマンドパレット等から呼び出されるクイック記録用のモーダル
- タイトル＋テンプレート選択＋本文入力の3ステップ

### `template-editor.ts`
- 設定画面からテンプレートを作成・編集する専用モーダル
- プレースホルダ補助ボタンや保存処理あり

---

## 🛠 `utils/path.ts`
- 保存先ディレクトリの生成確認・補助関数
- `getTemplateFolderPath()` や `ensureFolderExists()` などを実装

---

## 📄 `manifest.json`
```json
{
  "id": "quickdraft",
  "name": "QuickDraft",
  "version": "0.1.0",
  "minAppVersion": "0.15.0",
  "description": "Quickly capture structured ideas and specs using custom templates.",
  "author": "utakata",
  "main": "main.js"
}
````

---

## **✅ 今後の実装順（提案）**

1. settings.ts：設定型の定義とデフォルト値の整理
    
2. template/template-manager.ts：テンプレート読み書き処理の実装
    
3. template/placeholder.ts：プレースホルダ展開処理
    
4. ui/quickdraft-modal.ts：クイック記録UIの初期表示
    
5. settings-tab.ts + ui/template-editor.ts：テンプレート設定画面の実装
    

---

## **📌 備考**

- テンプレートは Markdown ファイルとして保存され、ユーザーにとっても編集可能な形式を維持する
    
- プレースホルダ設計は、将来的なカスタム関数対応も視野に入れて柔軟に
    
- 設定UIは初期はシンプルに、「テンプレート管理」が主眼
    
