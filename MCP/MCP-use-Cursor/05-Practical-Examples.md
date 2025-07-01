# MCPの実践的な使用例とベストプラクティス

## 実際の使用例

### 1. Web開発での統合例

#### GitHub + Notion + Linear統合
```json
{
  "mcpServers": {
    "github": {
      "command": "npx",
      "args": ["-y", "@smithery-ai/github"],
      "env": {
        "GITHUB_TOKEN": "${GITHUB_TOKEN}"
      }
    },
    "notion": {
      "command": "npx",
      "args": ["-y", "notion-mcp-server"],
      "env": {
        "NOTION_API_KEY": "${NOTION_API_KEY}"
      }
    },
    "linear": {
      "command": "npx",
      "args": ["-y", "linear-mcp-server"],
      "env": {
        "LINEAR_API_KEY": "${LINEAR_API_KEY}"
      }
    }
  }
}
```

**使用シナリオ**:
- NotionのPRD（Product Requirements Document）から要件を取得
- LinearからアサインされたタスクをCursorで実装
- 実装完了後、自動的にGitHubにプルリクエストを作成

#### 具体的なプロンプト例
```
"Linearからタスク#123の詳細を取得して、NotionのPRDセクション「ユーザー認証」を参照しながら、認証機能を実装してください。完了後、GitHubにプルリクエストを作成してください。"
```

### 2. データベース駆動開発

#### PostgreSQL + Redis統合
```json
{
  "mcpServers": {
    "postgres": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-postgres"],
      "env": {
        "POSTGRES_CONNECTION_STRING": "postgresql://user:pass@localhost/app_db"
      }
    },
    "redis": {
      "command": "npx",
      "args": ["-y", "redis-mcp-server"],
      "env": {
        "REDIS_URL": "redis://localhost:6379"
      }
    }
  }
}
```

**使用シナリオ**:
- データベーススキーマを分析してAPIエンドポイントを自動生成
- Redisキャッシュ戦略の最適化提案
- データベースクエリのパフォーマンス分析

### 3. DevOps統合例

#### AWS + Docker + Sentry統合
```json
{
  "mcpServers": {
    "aws": {
      "command": "npx",
      "args": ["-y", "aws-mcp-server"],
      "env": {
        "AWS_ACCESS_KEY_ID": "${AWS_ACCESS_KEY_ID}",
        "AWS_SECRET_ACCESS_KEY": "${AWS_SECRET_ACCESS_KEY}",
        "AWS_REGION": "us-west-2"
      }
    },
    "docker": {
      "command": "npx",
      "args": ["-y", "docker-mcp-server"]
    },
    "sentry": {
      "command": "uvx",
      "args": [
        "mcp-sentry",
        "--auth-token", "${SENTRY_TOKEN}",
        "--project-slug", "my-project",
        "--organization-slug", "my-org"
      ]
    }
  }
}
```

**使用シナリオ**:
- Sentryのエラーレポートを分析してバグ修正コードを提案
- Dockerfileの最適化とマルチステージビルドの実装
- AWS Lambdaのデプロイメント自動化

### 4. 研究・分析業務

#### Web検索 + データ可視化統合
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
        "{\"braveApiKey\":\"${BRAVE_API_KEY}\"}"
      ]
    },
    "puppeteer": {
      "command": "npx",
      "args": ["-y", "@agentdeskai/browser-tools-mcp"]
    }
  }
}
```

**使用シナリオ**:
- 競合他社の技術スタック調査とレポート生成
- 最新の技術トレンドの自動収集と分析
- Webサイトからデータを抽出してCSVファイルに整理

## ワークフロー最適化

### 1. プロジェクト固有の設定管理

#### 開発環境別設定
```bash
# 開発環境用
# .cursor/mcp.development.json
{
  "mcpServers": {
    "local-db": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-postgres"],
      "env": {
        "POSTGRES_CONNECTION_STRING": "postgresql://dev:dev@localhost/dev_db"
      }
    }
  }
}

# 本番環境用  
# .cursor/mcp.production.json
{
  "mcpServers": {
    "prod-db": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-postgres"],
      "env": {
        "POSTGRES_CONNECTION_STRING": "${PROD_DB_CONNECTION}"
      }
    }
  }
}
```

#### 設定切り替えスクリプト
```bash
#!/bin/bash
# switch-mcp-env.sh

ENV=$1
if [ "$ENV" = "dev" ]; then
    cp .cursor/mcp.development.json .cursor/mcp.json
    echo "Switched to development MCP configuration"
elif [ "$ENV" = "prod" ]; then
    cp .cursor/mcp.production.json .cursor/mcp.json
    echo "Switched to production MCP configuration"
else
    echo "Usage: ./switch-mcp-env.sh [dev|prod]"
fi
```

### 2. チーム開発での共有設定

#### 共有可能な設定テンプレート
```json
{
  "mcpServers": {
    "github": {
      "command": "npx",
      "args": ["-y", "@smithery-ai/github"],
      "env": {
        "GITHUB_TOKEN": "${GITHUB_TOKEN}"
      }
    },
    "linear": {
      "command": "npx",
      "args": ["-y", "linear-mcp-server"],
      "env": {
        "LINEAR_API_KEY": "${LINEAR_API_KEY}"
      }
    }
  }
}
```

#### 環境変数管理
```bash
# .env.example
GITHUB_TOKEN=your_github_token_here
LINEAR_API_KEY=your_linear_api_key_here
NOTION_API_KEY=your_notion_api_key_here

# チーム向けREADME.md
## MCP設定
1. `.env.example`を`.env`にコピー
2. 各APIキーを設定
3. `source .env`で環境変数を読み込み
4. Cursorを再起動
```

### 3. 自動化されたワークフロー

#### Puppeteerを使った自動テスト
```javascript
// プロンプト例
"Puppeteerを使って以下のテストを実行してください：
1. ログインページに移動
2. テストユーザーでログイン
3. ダッシュボードが正常に表示されることを確認
4. スクリーンショットを保存
5. 結果をレポートとして出力"
```

#### データパイプラインの自動化
```javascript
// プロンプト例  
"以下のデータパイプラインを実装してください：
1. 外部APIからデータを取得
2. PostgreSQLデータベースに保存
3. Redisにキャッシュを作成
4. データ品質チェックを実行
5. 問題があればSlackに通知"
```

## パフォーマンス最適化

### 1. MCPサーバーの効率的な使用

#### 必要に応じたサーバーの有効/無効化
```json
{
  "mcpServers": {
    "essential-github": {
      "command": "npx",
      "args": ["-y", "@smithery-ai/github"],
      "env": {
        "GITHUB_TOKEN": "${GITHUB_TOKEN}"
      }
    }
    // 他のサーバーは必要な時だけ有効化
  }
}
```

#### 条件付きサーバー起動
```bash
#!/bin/bash
# conditional-mcp-setup.sh

if [ "$PROJECT_TYPE" = "web" ]; then
    # Web開発用のMCPサーバーを設定
    cp .cursor/mcp.web.json .cursor/mcp.json
elif [ "$PROJECT_TYPE" = "ml" ]; then
    # ML開発用のMCPサーバーを設定
    cp .cursor/mcp.ml.json .cursor/mcp.json
fi
```

### 2. リソース監視と最適化

#### MCPサーバー監視スクリプト
```bash
#!/bin/bash
# monitor-mcp.sh

echo "=== MCP Server Status ==="
ps aux | grep -E "(npx.*mcp|uvx.*mcp)" | grep -v grep

echo "=== Memory Usage ==="
ps aux | grep -E "(npx.*mcp|uvx.*mcp)" | grep -v grep | awk '{print $4" "$11}'

echo "=== Network Connections ==="
lsof -i | grep -E "(npx|uvx)"
```

#### 自動最適化設定
```json
{
  "mcpServers": {
    "optimized-server": {
      "command": "npx",
      "args": ["-y", "your-server"],
      "env": {
        "NODE_OPTIONS": "--max-old-space-size=512",
        "MCP_CACHE_SIZE": "100MB",
        "MCP_TIMEOUT": "30000"
      }
    }
  }
}
```

## セキュリティのベストプラクティス

### 1. 機密情報の管理

#### 環境変数の暗号化
```bash
# sopsを使用した例
sops -e .env > .env.encrypted
git add .env.encrypted
echo ".env" >> .gitignore
```

#### APIキーのローテーション
```bash
#!/bin/bash
# rotate-api-keys.sh

# GitHub tokenの更新
gh auth refresh -h github.com -s repo

# 新しいtokenを環境変数に設定
export GITHUB_TOKEN=$(gh auth token)

# Cursorの再起動が必要
echo "Please restart Cursor to apply new API keys"
```

### 2. アクセス制御

#### 最小権限の原則
```json
{
  "mcpServers": {
    "readonly-github": {
      "command": "npx",
      "args": ["-y", "@smithery-ai/github"],
      "env": {
        "GITHUB_TOKEN": "${GITHUB_READONLY_TOKEN}"
      }
    }
  }
}
```

#### サンドボックス環境
```json
{
  "mcpServers": {
    "sandboxed-filesystem": {
      "command": "npx",
      "args": [
        "-y",
        "@modelcontextprotocol/server-filesystem",
        "/home/user/sandbox"
      ]
    }
  }
}
```

## 高度な使用例

### 1. カスタムMCPサーバーの開発

#### 基本的なMCPサーバー
```typescript
// custom-mcp-server.ts
import { FastMCP } from '@modelcontextprotocol/server-fast';

const mcp = new FastMCP('CustomServer');

@mcp.tool()
async function analyzeCode(code: string): Promise<string> {
  // カスタムコード分析ロジック
  return `Code analysis result for: ${code}`;
}

mcp.start();
```

#### カスタムサーバーの設定
```json
{
  "mcpServers": {
    "custom": {
      "command": "node",
      "args": ["./custom-mcp-server.js"]
    }
  }
}
```

### 2. 複雑なワークフローの自動化

#### CI/CD統合
```yaml
# .github/workflows/mcp-integration.yml
name: MCP Integration
on: [push]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - name: Setup MCP
        run: |
          cp .cursor/mcp.ci.json .cursor/mcp.json
          npm install -g @modelcontextprotocol/inspector
      - name: Test MCP Servers
        run: |
          mcp-inspector test
```

#### 多段階デプロイメント
```json
{
  "mcpServers": {
    "deployment": {
      "command": "npx",
      "args": ["-y", "deployment-mcp-server"],
      "env": {
        "DEPLOY_ENV": "${DEPLOY_ENV}",
        "AWS_ROLE": "${AWS_ROLE}"
      }
    }
  }
}
```

## まとめ

MCPをCursorで効果的に使用するためのポイント：

### 重要な原則
1. **段階的導入**: 1つずつMCPサーバーを追加してテスト
2. **適切な設定管理**: 環境別、プロジェクト別の設定を分離
3. **セキュリティ重視**: 機密情報の適切な管理
4. **パフォーマンス監視**: リソース使用量の定期的なチェック
5. **チーム連携**: 設定の共有と文書化

### 継続的改善
- 新しいMCPサーバーの定期的な調査
- ワークフローの継続的な最適化  
- セキュリティベストプラクティスの更新
- チームフィードバックの収集と反映
