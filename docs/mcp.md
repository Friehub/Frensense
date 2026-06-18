# MCP Server

Frensense ships with a **Model Context Protocol (MCP) server** — a lightweight JSON-RPC 2.0 interface over stdin/stdout that lets AI agents (Claude Code, Cursor, etc.) use Frensense as a first-class semantic analysis tool.

## Quick Start

```bash
# Build the MCP server
cargo build --features mcp

# Run it (reads JSON-RPC from stdin, writes responses to stdout)
./target/debug/frensense-mcp
```

## Supported Clients

Any MCP-compatible AI agent can connect to `frensense-mcp`. The server is transport-agnostic — it speaks JSON-RPC 2.0 over stdin/stdout, so it works with any MCP host that supports stdio-based servers.

### Claude Code

```json
{
  "mcpServers": {
    "frensense": {
      "command": "frensense-mcp",
      "args": []
    }
  }
}
```

### Cursor

In Cursor settings, add a new MCP server with:
- **Name:** `frensense`
- **Type:** `stdio`
- **Command:** `frensense-mcp`

## Exposed Tools

### `frensense_audit`

Run semantic analysis on a file or directory. This is the only tool the server exposes.

**Parameters:**

| Parameter | Type | Required | Default | Description |
|-----------|------|----------|---------|-------------|
| `path` | `string` | Yes | — | File or directory path to audit |
| `fix_auto` | `boolean` | No | `false` | Apply auto-fixable remediations in-place |
| `severity_threshold` | `string` | No | `"warning"` | Min severity: `"critical"`, `"warning"`, or `"info"` |

**Response:**

```json
{
  "clean": false,
  "advisories": [
    {
      "rule_id": "RUST_DEADLOCK_RISK",
      "severity": "Critical",
      "observation": "Mutex lock held across an await point",
      "impact": "Potential deadlock if another task tries to acquire the same lock",
      "improvement": "Restructure to drop the lock before awaiting",
      "line": 42,
      "column": 5,
      "file_path": "/project/src/handler.rs",
      "fingerprint": "sri://handler.rs/process_request/L42"
    }
  ],
  "auto_fixed": 0,
  "requires_human": [
    {
      "rule_id": "RUST_DEADLOCK_RISK",
      "severity": "Critical",
      ...
    }
  ]
}
```

**Fields:**
- `clean` — `true` if no advisories matched the threshold
- `advisories` — Array of findings (filtered by `severity_threshold`)
- `auto_fixed` — Count of advisories with available auto-fixes
- `requires_human` — Subset of advisories that need manual intervention (no `proposed_replacement` or `requires_human=true`)

### Examples

```json
// Request: audit a single file with default threshold
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "tools/call",
  "params": {
    "name": "frensense_audit",
    "arguments": {
      "path": "src/main.rs"
    }
  }
}

// Request: audit a directory, filter to critical only, apply auto-fixes
{
  "jsonrpc": "2.0",
  "id": 2,
  "method": "tools/call",
  "params": {
    "name": "frensense_audit",
    "arguments": {
      "path": "/project/src",
      "severity_threshold": "critical",
      "fix_auto": true
    }
  }
}
```

### Error Handling

When the tool encounters an error (e.g., path not found, analysis failure), the response includes an `error` field:

```json
{
  "clean": false,
  "advisories": [],
  "auto_fixed": 0,
  "requires_human": [],
  "error": "path does not exist: /nonexistent"
}
```

## Protocol Details

The server implements the standard MCP lifecycle:

1. **Initialize** — Client sends `initialize`, server returns protocol version and capabilities
2. **Initialized** — Client sends `notifications/initialized` notification
3. **Operation** — Client calls `tools/list` and `tools/call`
4. **Shutdown** — Client sends `shutdown`, then `exit`

### Supported Methods

| Method | Description |
|--------|-------------|
| `initialize` | Protocol handshake — returns version, capabilities, server info |
| `notifications/initialized` | Acknowledges initialization (no response) |
| `notifications/cancelled` | Cancels pending request (no response) |
| `shutdown` | Graceful shutdown — returns `null` |
| `exit` | Exits the process |
| `tools/list` | Returns the available tools (`frensense_audit`) |
| `tools/call` | Invokes `frensense_audit` with the given arguments |

### JSON-RPC 2.0 Compliance

- All requests use `"jsonrpc": "2.0"`
- Request IDs: string, number, `null`, or absent (notifications)
- Errors follow JSON-RPC 2.0 error codes:
  - `-32700` — Parse error
  - `-32600` — Invalid request
  - `-32601` — Method not found
  - `-32602` — Invalid params
  - `-32000` — Server error (startup failure)

## Diagnostics

The server writes diagnostic information to stderr:

```
frensense-mcp v0.3.0 starting
frensense-mcp: cwd=/project, rules=70, threshold="warning"
frensense-mcp: exiting
```

Clients can capture stderr for debugging without interfering with the JSON-RPC protocol on stdout.

## Troubleshooting

**Server exits immediately with no output:** Ensure `frensense-mcp` was built with the `mcp` feature. Build with `cargo build --features mcp` or `cargo build --features cli` (which includes `mcp`).

**"Method not found" on known method:** Verify the client is sending valid JSON-RPC 2.0 — the server requires `"jsonrpc": "2.0"` in every request.

**Path not found errors:** The MCP server resolves paths relative to its working directory. Use absolute paths or ensure the working directory is set correctly.
