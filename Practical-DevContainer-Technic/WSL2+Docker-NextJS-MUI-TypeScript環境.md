# WSL2+Docker で Next.js/MUI/TypeScript 開発環境構築

## 概要
WSL2+Docker環境でNext.js、Material UI、TypeScriptを組み合わせた開発環境を最も簡単に構築する方法。Material UI V5の公式examplesを活用した効率的なセットアップ手順。

## 著者
- **ユーザー**: yyos
- **記事URL**: https://qiita.com/yyos/items/409784a040d20a2c2d4a

## 実行環境
- Windows10
- WSL2: Ubuntu20.04LTS
- VSCode（Remote Development導入済み）

## 手順

### 1. WSL2+Docker環境でDevContainer作成

1. **VSCodeでDevContainer起動**
   - 左下の緑のアイコンをクリック
   - `Reopen in Container`を選択

2. **Node.js & TypeScript環境選択**
   - `Node.js & Typescript`を選択
   - Node.jsのバージョンを選択（例：14）
   - 追加インストール（Git等）を選択

3. **環境構築完了**
   - コンテナに接続された状態になる
   - `.devcontainer`フォルダが作成される

### DevContainer接続の操作
- **接続解除・再接続**: 左下の緑のアイコンから操作可能

### 2. Material UI公式Examplesの活用

**重要ポイント**: [MUI V5のexamples](https://mui.com/getting-started/example-projects/)には、V4にはなかった[Next.js (TypeScript version)](https://github.com/mui/material-ui/tree/master/examples/create-react-app-with-typescript)が追加されている

### 3. 開発環境セットアップ

構築したコンテナ環境でターミナルを開き、以下を実行：

```bash
# Material UIのNext.js TypeScript exampleをダウンロード・展開
curl https://codeload.github.com/mui/material-ui/tar.gz/master | tar -xz --strip=3 material-ui-master/examples/nextjs-with-typescript

# 依存関係インストール
yarn

# 開発サーバー起動
yarn dev
```

### コマンド解説
- `--strip=3`: カレントディレクトリ直下にソース展開するため（公式のREADMEでは`--strip=2`）
- 公式examplesのREADMEと基本的に同じ操作

## 最終形での拡張
記事では詳細を割愛しているが、以下のような拡張が可能：

1. **環境共有**
   - `.devcontainer`をリポジトリに含めてチーム開発環境を共有

2. **環境カスタマイズ**
   - `.devcontainer/devcontainer.json`
   - `.devcontainer/Dockerfile`
   - 上記ファイルの編集で環境をアレンジ可能

## ポイント
- Material UI V5の公式examplesが環境構築を劇的に簡単にした
- V4時代にはなかったNext.js TypeScript版テンプレートが利用可能
- WSL2 + Docker + DevContainerの組み合わせで効率的な開発環境構築が実現

## タグ
#WSL2 #Docker #NextJS #MaterialUI #TypeScript #DevContainer #VSCode #環境構築
