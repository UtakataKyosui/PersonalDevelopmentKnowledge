# MCPトラブルシューティングガイド

## 一般的な問題と解決方法

### 1. 接続エラー

#### 問題: "MCP error -32000: Connection closed"
**原因**: MCPサーバーの起動に失敗
**解決方法**:
```bash
# 1. パッケージが存在するか確認
npm list -g @modelcontextprotocol/server-filesystem

# 2. 手動でサーバーを起動してエラーを確認
npx -y @modelcontextprotocol/server-filesystem

# 3. 依存関係をクリア
npm cache clean --force
```

#### 問題: "MCP error -32001: Request timed out"
**原因**: サーバーレスポンスタイムアウト
**解決方法**:
- サーバーの起動時間を確認
- ネットワーク接続を確認
- サーバー側のログを確認

### 2. 環境・依存関係の問題

#### Node.js関連の問題
```bash
# Node.jsバージョン確認（10以上が推奨）
node --version

# npxが使用可能か確認
npx --version

# パッケージの再インストール
npm install -g npm@latest
```

#### Python関連の問題
```bash
# uvxが利用可能か確認
uvx --version

# Pythonバージョン確認
python --version

# 仮想環境の作成（必要に応じて）
python -m venv mcp-env
source mcp-env/bin/activate  # Linux/Mac
# mcp-env\Scripts\activate  # Windows
```

### 3. 設定ファイルの問題

#### mcp.jsonの構文エラー
```bash
# JSONファイルの構文チェック
cat .cursor/mcp.json | python -m json.tool

# または
node -pe "JSON.parse(require('fs').readFileSync('.cursor/mcp.json', 'utf8'))"
```

#### 一般的な設定ミス
```json
// ❌ 間違い
{
  "mcpServers": {
    "server-name": {
      "command": "npm",  // ❌ npmではなくnpx
      "args": ["@modelcontextprotocol/server-filesystem"]  // ❌ -yフラグなし
    }
  }
}

// ✅ 正しい
{
  "mcpServers": {
    "server-name": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-filesystem"]
    }
  }
}
```

### 4. 権限関連の問題

#### ファイルアクセス権限
```bash
# .cursorディレクトリの権限確認
ls -la .cursor/

# 権限修正（必要に応じて）
chmod 644 .cursor/mcp.json
chmod 755 .cursor/
```

#### API キー関連
```bash
# 環境変数の確認
echo $API_KEY

# 環境変数の設定（一時的）
export API_KEY="your-api-key-here"

# 永続的な設定（.bashrc, .zshrcなど）
echo 'export API_KEY="your-api-key-here"' >> ~/.bashrc
```

### 5. サーバー固有の問題

#### GitHubサーバー
```bash
# GitHubトークンの権限確認
curl -H "Authorization: token YOUR_TOKEN" https://api.github.com/user

# 必要な権限:
# - repo (リポジトリアクセス)
# - read:user (ユーザー情報)
```

#### PostgreSQLサーバー
```bash
# 接続文字列のテスト
psql "postgresql://user:pass@localhost/db"

# 接続権限の確認
GRANT ALL PRIVILEGES ON DATABASE yourdb TO youruser;
```

#### Braveサーチサーバー
```bash
# APIキーのテスト
curl -H "X-Subscription-Token: YOUR_API_KEY" \
     "https://api.search.brave.com/res/v1/web/search?q=test"
```

## デバッグ手順

### 1. ログの確認

#### Cursorのログ
1. Cursor → Help → Show Logs
2. MCP関連のエラーメッセージを確認
3. 重要なキーワード:
   - "Starting new stdio process with command"
   - "Client closed for command"
   - "Failed to reload client"

#### MCPサーバーのログ
```bash
# サーバーを手動実行してログ確認
npx -y @modelcontextprotocol/server-filesystem /path/to/files 2>&1 | tee mcp.log

# ログファイルの確認
tail -f mcp.log
```

### 2. 段階的テスト

#### ステップ1: 最小構成でテスト
```json
{
  "mcpServers": {
    "test": {
      "command": "echo",
      "args": ["hello"]
    }
  }
}
```

#### ステップ2: 公式サーバーでテスト
```json
{
  "mcpServers": {
    "filesystem": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-filesystem", "/tmp"]
    }
  }
}
```

#### ステップ3: 目的のサーバーでテスト
```json
{
  "mcpServers": {
    "target": {
      "command": "npx",
      "args": ["-y", "your-target-server"]
    }
  }
}
```

### 3. ネットワーク関連のトラブルシューティング

#### プロキシ設定
```bash
# npmプロキシ設定
npm config set proxy http://proxy-server:port
npm config set https-proxy http://proxy-server:port

# プロキシ設定の確認
npm config list
```

#### ファイアウォール
- ローカルで動作するMCPサーバーがファイアウォールでブロックされていないか確認
- 企業環境では特に注意が必要

## ベストプラクティス

### 1. 開発時のデバッグ

#### MCPサーバーのテスト環境
```bash
# MCP Inspector の使用
npx @modelcontextprotocol/inspector

# カスタムサーバーのテスト
npx @modelcontextprotocol/inspector npx -y your-mcp-server
```

#### ログレベルの設定
```json
{
  "mcpServers": {
    "debug-server": {
      "command": "npx",
      "args": ["-y", "your-server"],
      "env": {
        "LOG_LEVEL": "debug",
        "DEBUG": "*"
      }
    }
  }
}
```

### 2. パフォーマンス監視

#### 接続数の制限
- 同時接続するMCPサーバーは3つ以下に制限
- 不要なサーバーは無効化

#### リソース使用量の監視
```bash
# プロセス確認
ps aux | grep mcp

# メモリ使用量確認
top -p $(pgrep -f mcp)
```

### 3. セキュリティ対策

#### API キー管理
```bash
# 環境変数ファイルの作成
echo "API_KEY=your-secret-key" > .env
echo ".env" >> .gitignore

# 権限設定
chmod 600 .env
```

#### サーバー検証
- 信頼できるソースからのMCPサーバーのみ使用
- サーバーのソースコードを確認（可能であれば）
- 定期的なセキュリティ更新

## よくある質問（FAQ）

### Q: MCPサーバーが表示されない
**A**: 以下を確認：
1. mcp.jsonの構文が正しいか
2. Cursorの再起動
3. ファイルパスが正しいか

### Q: 「緑のドット」が表示されない
**A**: 
1. サーバーのログを確認
2. 依存関係のインストール確認
3. 権限とAPIキーの確認

### Q: 複数のMCPサーバーが動作しない
**A**:
1. サーバー数を3つ以下に制限
2. 各サーバーの設定を個別にテスト
3. リソース使用量の確認

### Q: SSH経由でMCPが動作しない
**A**: 
Cursor directly communicates with MCP servers from your local machine, either directly through stdio or via the network using sse. Therefore, MCP servers may not work properly when accessing Cursor over SSH or other development environments.

解決策:
1. ローカル環境でCursorを使用
2. ネットワーク経由のSSE接続を使用
3. リモート環境にMCPサーバーを直接設置

### Q: MCPサーバーの応答が遅い
**A**:
1. サーバーのパフォーマンスを確認
2. ネットワーク遅延をチェック
3. タイムアウト設定の調整
4. キャッシュの実装を検討

## トラブルシューティングチェックリスト

### 基本チェック
- [ ] Node.js/Pythonがインストールされているか
- [ ] mcp.jsonの構文が正しいか
- [ ] 必要なAPIキーが設定されているか
- [ ] ファイル権限が適切か
- [ ] Cursorが最新版か

### 詳細チェック
- [ ] サーバーが手動で起動するか
- [ ] ログにエラーメッセージがないか
- [ ] ネットワーク接続が正常か
- [ ] プロキシ設定が適切か
- [ ] ファイアウォールでブロックされていないか

### パフォーマンスチェック
- [ ] 同時接続サーバー数が3つ以下か
- [ ] リソース使用量が適切か
- [ ] 不要なサーバーが無効化されているか
- [ ] 定期的な監視が設定されているか

## 追加リソース

### 公式サポート
- [Cursor Community Forum](https://forum.cursor.com)
- [MCP Discord Server](https://discord.gg/mcp)
- [Cursor Documentation](https://docs.cursor.com)

### トラブルシューティングツール
- MCP Inspector: `npx @modelcontextprotocol/inspector`
- JSON Validator: オンラインJSON検証ツール
- Log Analyzers: システムログ分析ツール

### コミュニティリソース
- Stack Overflow (mcp, cursor タグ)
- GitHub Issues (各MCPサーバーのリポジトリ)
- Reddit (r/cursor, r/MachineLearning)
