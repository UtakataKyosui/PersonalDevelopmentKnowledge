# CursorでのMCP（Model Context Protocol）概要

## MCPとは何か

Model Context Protocol (MCP) is an open protocol that standardizes how applications provide context and tools to LLMs. Think of MCP as a plugin system for Cursor - it allows you to extend the Agent's capabilities by connecting it to various data sources and tools through standardized interfaces.

Model Context Protocol (MCP) is an open standard that connects AI models to external tools and data sources using one consistent method. It works through a client-server model where the AI sends requests to MCP servers that perform actions or fetch information.

### MCPの重要性

- **統一インターフェース**: MCP is like a USB-C port for your AI applications. Just as USB-C offers a standardized way to connect devices to various accessories, MCP standardizes how your AI apps connect to different data sources and tools.
- **統合の簡素化**: MCP allows you to connect Cursor to external systems and data sources. This means you can integrate Cursor with your existing tools and infrastructure, instead of having to tell Cursor what the structure of your project is outside of the code itself.
- **言語の柔軟性**: MCP servers can be written in any language that can print to stdout or serve an HTTP endpoint. This flexibility allows you to implement MCP servers using your preferred programming language and technology stack very quickly.

## アーキテクチャ

At its core, MCP follows a client-server architecture where a host application can connect to multiple servers.

### 主要コンポーネント

1. **Host**: Host represents any AI app (Claude desktop, Cursor) that provides an environment for AI interactions, accesses tools and data, and runs the MCP Client.
2. **MCP Client**: MCP Client operates within the host to enable communication with MCP servers.
3. **MCP Server**: MCP Server exposes specific capabilities and provides access to data

### MCPサーバーが提供する機能

1. **Tools**: Enable LLMs to perform actions through your server.
2. **Resources**: Expose data and content from your servers to LLMs.
3. **Prompts**: Create reusable prompt templates and workflows.

## CursorでのMCPサポート

Cursor supports three transport types for MCP servers: Each transport type has different use cases, with stdio being simpler for local development and SSE/Streamable HTTP offering more flexibility for distributed teams.

### 制限事項

- Some MCP servers, or user's with many MCP servers active, may have many tools available for Cursor to use. Currently, Cursor will only send the first 40 tools to the Agent.
- Cursor directly communicates with MCP servers from your local machine, either directly through stdio or via the network using sse. Therefore, MCP servers may not work properly when accessing Cursor over SSH or other development environments.

## 現在のMCPエコシステム

- 公式SDKが利用可能: TypeScript、Python、Java、Kotlin、C#
- 豊富なプリビルトサーバー: GitHub、Google Drive、Slack、PostgreSQLなど
- コミュニティによる継続的な開発とサポート
- You can now set up MCP servers in Cursor with one click! We've curated a collection of popular MCP servers that you can install instantly with OAuth support for quick authentication.

## 関連リンク

- [公式MCP仕様](https://modelcontextprotocol.io)
- [Cursor MCP ドキュメント](https://docs.cursor.com/context/model-context-protocol)
- [MCPサーバーディレクトリ](https://cursor.directory/mcp)
- [Awesome MCP Servers](https://github.com/awesome-mcp-servers)
