<div align="center">

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?logo=rust&logoColor=white)](https://crates.io/crates/hyper-mcp)
[![License](https://img.shields.io/badge/License-Apache--2.0-blue)](#license)
[![Issues - daytona](https://img.shields.io/github/issues/tuananh/hyper-mcp)](https://github.com/tuananh/hyper-mcp/issues)
<!-- ![GitHub Release](https://img.shields.io/github/v/release/tuananh/hyper-mcp) -->

</div>

<div align="center">
  <picture>
    <img alt="hyper-mcp logo" src="./assets/logo.png" width="50%">
  </picture>
</div>

# hyper-mcp

A fast, secure MCP server that extends its capabilities through WebAssembly plugins.

## What is it?

hyper-mcp makes it easy to add AI capabilities to your applications. It works with Claude Desktop, Cursor IDE, and other MCP-compatible apps. Write plugins in your favorite language, distribute them through container registries, and run them anywhere - from cloud to edge.

## Features

- Write plugins in any language that compiles to WebAssembly
- Distribute plugins via standard OCI registries (like Docker Hub)
- Built on [Extism](https://github.com/extism/extism) for rock-solid plugin support
- Lightweight enough for resource-constrained environments
- Deploy anywhere: serverless, edge, mobile, IoT devices
- Cross-platform compatibility out of the box

## Security

Built with security-first mindset:

- Sandboxed plugins that can't access your system without permission
- Memory-safe execution with resource limits
- Secure plugin distribution through container registries
- Fine-grained access control for host functions
- OCI plugin images are signed at publish time and verified at load time with [sigstore](https://www.sigstore.dev/).

## Getting Started

1. Create your config file:
   - Linux: `$HOME/.config/hyper-mcp/config.json`
   - Windows: `{FOLDERID_RoamingAppData}`. Eg: `C:\Users\Alice\AppData\Roaming`
   - macOS: `$HOME/Library/Application Support/hyper-mcp/config.json`

```json
{
  "plugins": [
    {
      "name": "time",
      "path": "oci://ghcr.io/tuananh/time-plugin:latest"
    },
    {
      "name": "qr-code",
      "path": "oci://ghcr.io/tuananh/qrcode-plugin:latest"
    },
    {
      "name": "hash",
      "path": "oci://ghcr.io/tuananh/hash-plugin:latest"
    },
    {
      "name": "myip",
      "path": "oci://ghcr.io/tuananh/myip-plugin:latest",
      "runtime_config": {
        "allowed_hosts": ["1.1.1.1"]
      }
    },
    {
      "name": "fetch",
      "path": "oci://ghcr.io/tuananh/fetch-plugin:latest",
      "runtime_config": {
        "allowed_hosts": ["*"]
      }
    }
  ]
}
```

2. Start the server:

```sh
$ hyper-mcp
```

By default, logs will go into [state_dir](https://docs.rs/dirs/6.0.0/dirs/fn.state_dir.html) on Linux or [data_local_dir](https://docs.rs/dirs/6.0.0/dirs/fn.data_local_dir.html) on macOS & Windows.

| Platform | Value | Example |
|----------|--------|---------|
| Linux | `$XDG_DATA_HOME` or `$HOME/.local/share` | `/home/alice/.local/share` |
| macOS | `$HOME/Library/Application Support` | `/Users/Alice/Library/Application Support` |
| Windows | `{FOLDERID_LocalAppData}` | `C:\Users\Alice\AppData\Local` |

If you're loading unsigned OCI plugin, you will need to use `--insecure-skip-signature` flag.

## Using with Cursor IDE

You can configure hyper-mcp either globally for all projects or specifically for individual projects.

1. For project-scope configuration, create `.cursor/mcp.json` in your project root:
```json
{
  "mcpServers": {
    "hyper-mcp": {
      "command": "/path/to/hyper-mcp",
      "args": [""]
    }
  }
}
```

2. Set up hyper-mcp in Cursor's settings:
   ![cursor mcp](./assets/cursor-mcp.png)

3. Start using tools through chat:
   ![cursor mcp chat](./assets/cursor-mcp-1.png)

## Available Plugins

We maintain several example plugins to get you started:

- [time](https://github.com/tuananh/hyper-mcp/tree/main/examples/plugins/time): Get current time and do time calculations (Rust)
- [qr-code](https://github.com/tuananh/hyper-mcp/tree/main/examples/plugins/qr-code): Generate QR codes (Rust)
- [hash](https://github.com/tuananh/hyper-mcp/tree/main/examples/plugins/hash): Generate various types of hashes (Rust)
- [myip](https://github.com/tuananh/hyper-mcp/tree/main/examples/plugins/myip): Get your current IP (Rust)
- [fetch](https://github.com/tuananh/hyper-mcp/tree/main/examples/plugins/fetch): Basic webpage fetching (Rust)
- [crypto-price](https://github.com/tuananh/hyper-mcp/tree/main/examples/plugins/crypto-price): Get cryptocurrency prices (Go)
- [fs](https://github.com/tuananh/hyper-mcp/tree/main/examples/plugins/fs): File system operations (Rust)
- [github](https://github.com/tuananh/hyper-mcp/tree/main/examples/plugins/fs): GitHub plugin (Go)
- [eval-py](https://github.com/tuananh/hyper-mcp/tree/main/examples/plugins/eval-py): Evaluate Python code with RustPython (Rust)
- [arxiv](https://github.com/tuananh/hyper-mcp/tree/main/examples/plugins/arxiv): Search & download arXiv papers (Rust)
- [memory](https://github.com/tuananh/hyper-mcp/tree/main/examples/plugins/memory): Let you store & retrive memory, powered by SQLite (Rust)
- [sqlite](https://github.com/tuananh/hyper-mcp/tree/main/examples/plugins/memory): Interact with SQLite (Rust)

### Community-built plugins

- [hackernews](https://github.com/hungran/hyper-mcp-hackernews-tool): This plugin connects to the Hacker News API to fetch the current top stories and display them with their titles, scores, authors, and URLs.
- [release-monitor-id](https://github.com/ntheanh201/hyper-mcp-release-monitor-id-tool): This plugin retrieves project ID from release-monitoring.org, which helps track versions of released software.
- [yahoo-finance](https://github.com/phamngocquy/hyper-mcp-yfinance): This plugin connects to the Yahoo Finance API to provide stock prices (OHLCV) based on a company name or ticker symbol.

## Creating Plugins

Check out our [example plugins](https://github.com/tuananh/hyper-mcp/tree/main/examples/plugins) to learn how to build your own.

To publish a plugin:

```dockerfile
# example how to build with rust
FROM rust:1.86-slim AS builder

RUN rustup target add wasm32-wasip1 && \
    rustup component add rust-std --target wasm32-wasip1

WORKDIR /workspace
COPY . .
RUN cargo fetch
RUN cargo build --release --target wasm32-wasip1

# copy wasm to final image
FROM scratch
WORKDIR /
COPY --from=builder /workspace/target/wasm32-wasip1/release/your-plugin.wasm /plugin.wasm
```

Then build and push:
```sh
docker build -t your-registry/plugin-name .
docker push your-registry/plugin-name
```

## License

[Apache 2.0](./LICENSE)

## Star History

[![Star History Chart](https://api.star-history.com/svg?repos=tuananh/hyper-mcp&type=Date)](https://www.star-history.com/#tuananh/hyper-mcp&Date)