# Windows環境でのdevcontainer + Claude Code構築

> **原文**: [devcontainerでClaude Codeを動かす(Windows)](https://zenn.dev/acntechjp/articles/fc111da7542e00)

## 概要

Windows環境でClaude CodeをDevContainerで隔離実行する方法。AI Agentによる予期せぬファイルやライブラリの変更リスクを回避し、安全な開発環境を構築。

## アーキテクチャ

```
Windows 11 Pro
├── Podman (Docker Desktop代替)
├── VS Code + DevContainers Extension
└── DevContainer
    └── Claude Code (隔離環境)
```

## 環境構築手順

### 1. 基本ツールのインストール

#### Git
- [公式サイト](https://git-scm.com/downloads)からダウンロード
- PowerShellから`git --version`で確認

#### VS Code
- [公式サイト](https://code.visualstudio.com/download)からダウンロード

#### Node.js
- [公式サイト](https://nodejs.org/ja/download)からダウンロード

### 2. Podmanインストール・設定

#### Podmanダウンロード
- [Podman公式](https://podman.io/)から「Podman CLI for Windows」をダウンロード
- インストール時は「Windows Linux Subsystems (WSLv2)」を選択

#### 仮想マシン構築
```powershell
# 仮想マシン作成
podman machine init claudeVM

# 仮想マシン起動
podman machine start claudeVM

# デフォルト接続設定確認・変更
podman system connection list
podman system connection default claudeVM  # 必要に応じて
```

### 3. DevContainersセットアップ

#### DevContainers CLIインストール
```bash
npm install -g @devcontainers/cli
```

#### Claude Codeクローン
```bash
git clone https://github.com/anthropics/claude-code.git
cd claude-code
```

#### .devcontainerファイルのコピー
Claude Codeリポジトリの`.devcontainer`フォルダを対象プロジェクトフォルダにコピー：

```
.devcontainer/
├── devcontainer.json
├── Dockerfile
└── init-firewall.sh
```

### 4. DevContainer構築・起動

#### コンテナビルド
```powershell
# VS Codeで対象プロジェクトを開いた状態で実行
devcontainer up --workspace-folder . --docker-path podman
```

#### コンテナアクセス
```powershell
# コンテナID確認
podman ps

# コンテナ内にアクセス
podman exec -it <CONTAINER_ID> zsh
```

#### 効率化スクリプト
```powershell
# 現在のワークスペースフォルダのパス取得
$currentFolder = (Get-Location).Path

# そのフォルダのラベルを持つコンテナのIDを検索
$containerId = podman ps --filter "label=devcontainer.local_folder=$currentFolder" --format "{{.ID}}"

# 取得したコンテナIDでpodman exec実行
podman exec -it $containerId zsh
```

### 5. Claude Code起動
```bash
# devcontainer内で実行
claude
```

## 自動化PowerShellスクリプト

### Claude-Code.ps1（UTF-8 BOM付きで保存）
```powershell
# スクリプトの開始を通知
Write-Host "--- DevContainer 起動・接続スクリプト ---"

# Step 1: Podman machine の起動
Write-Host "Podman machine 'claudeVM' を起動しています..."
try {
    podman machine start claudeVM -q
    Write-Host "Podman machine 起動済みまたは起動しました。"
} catch {
    Write-Error "Podman machine の起動に失敗しました: $($_.Exception.Message)"
    exit 1
}

# Step 2: デフォルトコネクションの設定
Write-Host "デフォルトの Podman コネクションを 'claudeVM' に設定しています..."
try {
    podman system connection default claudeVM
    Write-Host "デフォルトコネクションを設定しました。"
} catch {
    Write-Warning "デフォルトの Podman コネクション設定に失敗しました: $($_.Exception.Message)"
}

# Step 3: DevContainer の起動
Write-Host "現在のフォルダで DevContainer を起動しています..."
try {
    devcontainer up --workspace-folder . --docker-path podman
    Write-Host "DevContainer の起動処理が完了しました。"
} catch {
    Write-Error "DevContainer の起動に失敗しました: $($_.Exception.Message)"
    exit 1
}

# Step 4: DevContainer のコンテナIDを取得
Write-Host "DevContainer のコンテナIDを検索しています..."
$currentFolder = (Get-Location).Path
$containerId = $(podman ps --filter "label=devcontainer.local_folder=$currentFolder" --format "{{.ID}}").Trim()

if (-not $containerId) {
    Write-Error "現在のフォルダ ('$currentFolder') に対応する DevContainer のコンテナIDが見つかりませんでした。"
    exit 1
}

Write-Host "コンテナIDが見つかりました: $containerId"

# Step 5: コンテナ内でコマンドを実行し、インタラクティブシェルに入る
Write-Host "コンテナ ($containerId) 内で 'claude' コマンドを実行し、その後 zsh セッションを開始します..."
try {
    podman exec -it $containerId zsh -c 'claude; exec zsh'
    Write-Host "インタラクティブセッションが終了しました。"
} catch {
    Write-Error "コンテナ内でのコマンド実行に失敗しました: $($_.Exception.Message)"
    exit 1
}

Write-Host "--- スクリプト完了 ---"
```

### 実行方法

#### フォルダ構成
```
Project/
├── .devcontainer/
│   ├── devcontainer.json
│   ├── Dockerfile
│   └── init-firewall.sh
└── Script/
    └── Claude-Code.ps1
```

#### 実行手順
```powershell
# Projectフォルダに移動
cd C:\Users\YourUser\Documents\Project

# スクリプト実行
.\Script\Claude-Code.ps1
```

## メリット

### セキュリティ
- Claude Codeがコンテナ内で隔離実行
- ホストシステムへの影響を最小限に抑制
- 予期せぬファイル変更や依存関係変更を防止

### 一貫性
- 開発環境の標準化
- チーム間での環境差異を排除
- 再現可能な開発環境

### 管理効率
- ワンクリックでの環境構築
- 自動化されたセットアップ
- VS Code統合による開発体験

## 注意点

- **Docker Desktop制限**: 企業環境でDocker Desktopが使用できない場合のPodman代替
- **リソース消費**: コンテナ実行によるシステムリソース使用
- **学習コスト**: DevContainer概念の理解が必要

## まとめ

Windows環境でのClaude Code安全実行を実現するDevContainer構築方法。Podmanを使用することでライセンス制約を回避し、コンテナ隔離によりセキュリティを確保。自動化スクリプトにより効率的な開発環境を提供。