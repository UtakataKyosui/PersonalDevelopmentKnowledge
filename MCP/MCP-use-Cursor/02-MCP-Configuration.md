# CursorでのMCP設定方法

## 基本的な設定手順

### 1. 前提条件
MCPサーバーを使用する前に、以下のツールがインストールされていることを確認してください：
- **Node.js** (npx付き): ほとんどのMCPサーバーで必要
- **Python** (uvx付き): Python系MCPサーバーで必要

### 2. MCP設定ファイルの作成

CursorでMCPサーバーを設定するには、`mcp.json`ファイルを作成する必要があります。

#### 設定場所の選択

**プロジェクト固有の設定:**
```
.cursor/mcp.json
```
Create a .cursor directory in your project root if it doesn't exist. Create a .cursor/mcp.json file for tools specific to that project.

**グローバル設定:**
```
~/.cursor/mcp.json
```
Create a file in your home directory for tools you want to use across all projects.

### 3. mcp.jsonの基本構造

```json
{
  "mcpServers": {
    "server-name": {
      "command": "npx",
      "args": ["-y", "mcp-server-package"],
      "env": {
        "API_KEY": "your-api-key-here"
      }
    }
  }
}
```

### 4. トランスポートタイプ

#### stdio（標準入出力）
最もシンプルで、ローカル開発に適している：
```json
{
  "mcpServers": {
    "file-system": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-script"]
    }
  }
}
```

#### SSE（Server-Sent Events）
ネットワーク経由でのアクセスに適している：
```json
{
  "mcpServers": {
    "server-name": {
      "url": "http://localhost:3000/sse",
      "env": {
        "API_KEY": "value"
      }
    }
  }
}
```

#### Python CLIサーバー
```json
{
  "mcpServers": {
    "sentry": {
      "command": "uvx",
      "args": [
        "mcp-sentry", 
        "--auth-token", "YOUR_SENTRY_TOKEN",
        "--project-slug", "YOUR_PROJECT_SLUG",
        "--organization-slug", "YOUR_ORGANIZATION_SLUG"
      ]
    }
  }
}
```

## 実際の設定例

### 1. ファイルシステムMCPサーバー
```json
{
  "mcpServers": {
    "file-system": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-script"]
    }
  }
}
```

### 2. SupabaseMCPサーバー
```json
{
  "mcpServers": {
    "supabase": {
      "command": "npx",
      "args": [
        "-y",
        "@supabase/mcp-server-supabase@latest",
        "--access-token",
        "<personal-access-token>"
      ]
    }
  }
}
```

### 3. Brave Search MCPサーバー
```json
{
  "mcpServers": {
    "brave-search": {
      "command": "npx",
      "args": [
        "-y",
        "@smithery/cli@latest",
        "run",
        "@smithery-ai/brave-search",
        "--config",
        "{\"braveApiKey\":\"YOUR_API_KEY\"}"
      ]
    }
  }
}
```

## Cursor設定の確認

### 1. 設定画面での確認
1. Cursorを開く
2. Settings → MCP に移動
3. 緑のアクティブステータスが表示されることを確認

### 2. サーバーステータス確認
- 緑のドット: サーバーが正常に動作中
- 赤のドット: サーバーに問題がある
- グレーのドット: サーバーが無効

## ベストプラクティス

### セキュリティ
- API キーなどの機密情報は環境変数で管理
- プロダクション環境では適切なセキュリティ対策を実装
- 信頼できるMCPサーバーのみを使用

### パフォーマンス
- 同時に接続するMCPサーバーは3つ以下に制限することを推奨
- サーバーのヘルスとパフォーマンスメトリクスを定期的に監視
- 不要なサーバーは無効化

### 信頼性
- MCPサーバーの依存関係を定期的に更新
- 設定ファイルをチームで共有する場合は、機密情報を除外
- バックアップ設定を準備

## 設定ファイルの管理

### チーム共有
```bash
# 設定をgitで管理する場合
git add .cursor/mcp.json

# 機密情報を除外する場合
echo ".cursor/mcp.json" >> .gitignore
```

### 環境変数の使用
```json
{
  "mcpServers": {
    "github": {
      "command": "npx",
      "args": ["-y", "@smithery-ai/github"],
      "env": {
        "GITHUB_TOKEN": "${GITHUB_TOKEN}"
      }
    }
  }
}
```
