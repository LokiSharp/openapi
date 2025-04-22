# LongPort MCP

A [MCP](https://modelcontextprotocol.io/introduction) server implementation for [LongPort OpenAPI](https://open.longportapp.com), provides real-time stock market data, provides AI access analysis and trading capabilities through MCP.

## Features

- Trading - Create, amend, cancel orders, query today’s/past orders and transaction details, etc.
- Quotes - Real-time quotes, acquisition of historical quotes, etc.
- Portfolio - Real-time query of the account assets, positions, funds

## Installation

## macOS or Linux

Run script to install:

```bash
curl -sSL https://raw.githubusercontent.com/longportapp/openapi/refs/heads/main/mcp/install | bash
```

## Windows

Download the latest binary from the [Releases](https://github.com/longportapp/openapi/releases/tag/longport-mcp-0.1.0) page.

## Running on Cursor

To configure LongPort MCP in Cursor:

- Open Cursor Settings
- Go to Features > MCP Servers
- Click `+ Add New MCP Server`
- Enter the following:
  - Name: `longport-mcp` (or your preferred name)
  - Type: `command`
  - Command: `env LONGPORT_APP_KEY=your-app-key LONGPORT_APP_SECRET=your-app-secret LONGPORT_ACCESS_TOKEN=your-access-token longport-mcp`

If you are using Windows, replace command with `cmd /c "set LONGPORT_APP_KEY=your-app-key && set LONGPORT_APP_SECRET=your-app-secret && set LONGPORT_ACCESS_TOKEN=your-access-token && longport-mcp"`

## Running on Cherry Studio

To configure LongPort MCP in Cherry Studio:

- Go to Settings > MCP Servers
- Click `+ Add Server`
- Enter the following:
  - Name: `longport-mcp` (or your preferred name)
  - Type: `STDIO`
  - Command: `env LONGPORT_APP_KEY=your-app-key LONGPORT_APP_SECRET=your-app-secret LONGPORT_ACCESS_TOKEN=your-access-token longport-mcp`

If you are using Windows, replace command with `cmd /c "set LONGPORT_APP_KEY=your-app-key && set LONGPORT_APP_SECRET=your-app-secret && set LONGPORT_ACCESS_TOKEN=your-access-token && longport-mcp"`

## Running as a SSE server

```bash
env LONGPORT_APP_KEY=your-app-key LONGPORT_APP_SECRET=your-app-secret LONGPORT_ACCESS_TOKEN=your-access-token longport-mcp --sse
```

Default bind address is `127.0.0.1:8000`, you can change it by using the `--bind` flag:

```bash
longport-mcp --sse --bind 127.0.0.1:3000
```

## Configuration

### Enable logging

To enable logging, set the flag `--log-dir` to the directory where you want to store the logs:

```bash
longport-mcp --log-dir /path/to/log/dir
```
