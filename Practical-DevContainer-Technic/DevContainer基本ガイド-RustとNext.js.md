# DevContainer基本ガイド - RustとNext.js

## 概要
DevContainerを使用したRustとNext.jsの環境構築の実演記事。環境構築の手間を省き、ローカル環境を汚さずに開発環境を構築する方法を紹介。

## 著者
- **ユーザー**: s1270070
- **記事URL**: https://qiita.com/s1270070/items/e7f317dfa60bf45a628c

## 前提条件
### 必要なもの
- VSCode
- Docker環境

### 環境
- macOS 13.6.1
- colima version 0.6.6
- Docker version 24.0.7

## 手順

### 準備
1. DevContainerの拡張機能をVSCodeにインストール

### Rust環境構築

1. **プロジェクトディレクトリ作成**
   ```bash
   mkdir ~/rust_sample && code $_
   ```

2. **DevContainer設定ファイル生成**
   - `Cmd+Shift+P`でコマンドパレット表示
   - `Dev Container: Add Dev Container ...`を選択
   - `rust`を検索して選択
   - `.devcontainer/devcontainer.json`が生成される

3. **DevContainer起動**
   - 右下の「Reopen in Container」をクリック
   - または左下の緑ボタンから「Reopen in Container」を選択

4. **Rust開発開始**
   ```bash
   cargo init
   cargo run
   ```

### Next.js環境構築

1. **プロジェクトディレクトリ作成**
   ```bash
   mkdir ~/nextjs && code $_
   ```

2. **DevContainer設定ファイル生成**
   - コマンドパレットで`Dev Container: Add Dev Container ...`を選択
   - `node`を検索して選択

3. **DevContainer起動**
   - 「Reopen in Container」を実行

4. **Next.js開発開始**
   ```bash
   # DevContainer設定を一時退避
   sudo mv .devcontainer/ ../
   
   # Next.jsサンプルアプリ作成
   sudo npx create-next-app -e with-next-ui .
   
   # DevContainer設定を復元
   sudo mv ../.devcontainer .
   
   # サーバー起動
   npm run dev
   ```

## ポイント
- DevContainerには多様な環境テンプレートが用意されている
- 初回起動時はイメージのプルに時間がかかる場合がある
- ローカル環境を汚さずに様々な開発環境を試せる

## タグ
#DevContainer #VSCode #Docker #Rust #NextJS #環境構築
