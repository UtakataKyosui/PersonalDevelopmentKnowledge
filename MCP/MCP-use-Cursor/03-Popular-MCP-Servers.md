# 人気のMCPサーバー一覧

## 公式・推奨MCPサーバー

### 1. ファイルシステムサーバー
**パッケージ**: `@modelcontextprotocol/server-filesystem`
**用途**: ローカルファイルシステムへのアクセス
```json
{
  "mcpServers": {
    "filesystem": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-filesystem", "/path/to/allowed/files"]
    }
  }
}
```

### 2. GitHub統合
**パッケージ**: `@smithery-ai/github`
**用途**: GitHubリポジトリ管理、ファイル操作、API統合
```json
{
  "mcpServers": {
    "github": {
      "command": "npx",
      "args": ["-y", "@smithery-ai/github"],
      "env": {
        "GITHUB_TOKEN": "your-github-token"
      }
    }
  }
}
```

### 3. Slack統合
**用途**: チャンネル管理、メッセージ送信
```json
{
  "mcpServers": {
    "slack": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-slack"],
      "env": {
        "SLACK_BOT_TOKEN": "your-slack-token"
      }
    }
  }
}
```

## データベース関連

### 4. PostgreSQL
**パッケージ**: `@modelcontextprotocol/server-postgres`
**用途**: PostgreSQLデータベースクエリと管理
```json
{
  "mcpServers": {
    "postgres": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-postgres"],
      "env": {
        "POSTGRES_CONNECTION_STRING": "postgresql://user:pass@localhost/db"
      }
    }
  }
}
```

### 5. Supabase
**パッケージ**: `@supabase/mcp-server-supabase`
**用途**: Supabaseプロジェクトとの統合
```json
{
  "mcpServers": {
    "supabase": {
      "command": "npx",
      "args": [
        "-y",
        "@supabase/mcp-server-supabase@latest",
        "--access-token",
        "your-access-token"
      ]
    }
  }
}
```

### 6. Redis
**用途**: Redisキー値ストアとの連携
```json
{
  "mcpServers": {
    "redis": {
      "command": "npx",
      "args": ["-y", "mcp-server-redis"],
      "env": {
        "REDIS_URL": "redis://localhost:6379"
      }
    }
  }
}
```

## Web・API関連

### 7. Web検索（Brave Search）
**パッケージ**: `@smithery-ai/brave-search`
**用途**: Web検索機能
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

### 8. Firecrawl（Web スクレイピング）
**用途**: 強力なWebスクレイピング機能
```json
{
  "mcpServers": {
    "firecrawl": {
      "command": "npx",
      "args": ["-y", "@firecrawl/mcp-server"],
      "env": {
        "FIRECRAWL_API_KEY": "your-api-key"
      }
    }
  }
}
```

### 9. Puppeteer（ブラウザ自動化）
**用途**: ブラウザ制御とスクリーンショット
```json
{
  "mcpServers": {
    "puppeteer": {
      "command": "npx",
      "args": ["-y", "@agentdeskai/browser-tools-mcp"]
    }
  }
}
```

## プロダクティビティツール

### 10. Notion
**用途**: Notionページアクセスと管理
```json
{
  "mcpServers": {
    "notion": {
      "command": "npx",
      "args": ["-y", "notion-mcp-server"],
      "env": {
        "NOTION_API_KEY": "your-notion-key"
      }
    }
  }
}
```

### 11. Linear（プロジェクト管理）
**用途**: Linear チケット管理
```json
{
  "mcpServers": {
    "linear": {
      "command": "npx",
      "args": ["-y", "linear-mcp-server"],
      "env": {
        "LINEAR_API_KEY": "your-linear-key"
      }
    }
  }
}
```

### 12. Cal.com（カレンダー）
**用途**: カレンダー統合
```json
{
  "mcpServers": {
    "cal": {
      "command": "npx",
      "args": ["-y", "cal-mcp-server"],
      "env": {
        "CAL_API_KEY": "your-cal-key"
      }
    }
  }
}
```

## 開発ツール

### 13. Docker
**用途**: Dockerコンテナ管理
```json
{
  "mcpServers": {
    "docker": {
      "command": "npx",
      "args": ["-y", "docker-mcp-server"]
    }
  }
}
```

### 14. AWS
**用途**: AWSサービス管理
```json
{
  "mcpServers": {
    "aws": {
      "command": "npx",
      "args": ["-y", "aws-mcp-server"],
      "env": {
        "AWS_ACCESS_KEY_ID": "your-key",
        "AWS_SECRET_ACCESS_KEY": "your-secret"
      }
    }
  }
}
```

### 15. Sentry（エラー監視）
**パッケージ**: `mcp-sentry`
**用途**: Sentryからエラー情報を取得・分析
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

## 特殊用途

### 16. Peekaboo（スクリーンショット）
**用途**: macOS用スクリーンショット機能
```json
{
  "mcpServers": {
    "peekaboo": {
      "command": "npx",
      "args": ["-y", "peekaboo-mcp"]
    }
  }
}
```

### 17. Spotify API
**用途**: Spotify音楽データアクセス
```json
{
  "mcpServers": {
    "spotify": {
      "command": "npx",
      "args": ["-y", "spotify-mcp-server"],
      "env": {
        "SPOTIFY_CLIENT_ID": "your-client-id",
        "SPOTIFY_CLIENT_SECRET": "your-client-secret"
      }
    }
  }
}
```

### 18. Sequential Thinking
**用途**: 段階的思考プロセス支援
```json
{
  "mcpServers": {
    "sequential-thinking": {
      "command": "npx",
      "args": ["-y", "sequential-thinking-mcp"]
    }
  }
}
```

## ComposioによるMCPサーバー

### 19. Composio統合プラットフォーム
**用途**: 100+のサービスとの統合
```bash
# インストール
npx @composio/cli@latest add cursor

# 利用可能なツール確認
npx @composio/cli@latest apps
```

## リソース

### MCPサーバーを見つけるためのリソース
1. **公式例**: [modelcontextprotocol.io](https://modelcontextprotocol.io)
2. **Awesome MCP Servers**: [GitHub Repository](https://github.com/awesome-mcp-servers)
3. **Cursor Directory**: [cursor.directory/mcp](https://cursor.directory/mcp)
4. **Smithery.ai**: ワンクリックでMCPサーバーを見つけてデプロイ
5. **MCP.so**: MCPサーバーの最大コレクション

### MCPサーバー開発
- **公式SDK**: TypeScript、Python、Java、Kotlin、C#
- **テストツール**: MCP Inspector
- **コミュニティサポート**: MCP Discord Server
