# hyper-mcp

> Build, publish & run your Model Context Protocol (MCP) applet with ease.

<p align="center">
  <img src="./assets/ai.jpg" style="height: 300px;">
</p>

## Overview

hyper-mcp enables you to create and run MCP plugins in any programming language that compiles to WebAssembly. It integrates seamlessly with OCI registries for distribution and works with popular MCP-compatible applications like Claude Desktop and Cursor IDE.

## Why hyper-mcp?

### WebAssembly-First
- Easy local runtime embedding
- First-class support in many programming languages
- Leverages Extism's PDKs for simplified plugin development

### OCI Registry Integration
- Uses existing container infrastructure
- Familiar packaging workflow with Dockerfiles
- Enables self-hosting for enterprise environments

## Key Features

- **Language Agnostic**: Build plugins in any language that compiles to WebAssembly
- **Simple Distribution**: Package plugins using Dockerfile and publish to any OCI registry
- **Universal Compatibility**: Works with any MCP-compatible application
- **Easy Configuration**: Add new tools by simply editing a config file and restarting the MCP server

## Quick Start

1. Create a configuration file at `$HOME/.config/mcp.json`:

```json
{
  "plugins": [
    {
      "name": "time",
      "path": "/home/anh/Code/hyper-mcp/wasm/time.wasm"
    },
    {
      "name": "qr-code",
      "path": "oci://ghcr.io/tuananh/qrcode-plugin:latest"
    },
    {
      "name": "hash",
      "path": "oci://ghcr.io/tuananh/hash-plugin:latest"
    }
  ]
}
```

The `path` can be:
- A local file path
- An HTTP URL
- An OCI image reference

2. Start the server:

```sh
$ hyper-mcp
```

## Integration Guides

### Cursor IDE Integration

1. Configure Cursor to use hyper-mcp:

![cursor mcp](./assets/cursor-mcp.png)

2. Access tools through Cursor's chat UI:

![cursor mcp chat](./assets/cursor-mcp-1.png)

### Claude Desktop Integration

Documentation coming soon for Windows/macOS users.

## Building Plugins

hyper-mcp uses [Extism](https://github.com/extism/extism) for plugin development. Check out our example plugins:
- [QR Code Plugin](https://github.com/tuananh/hyper-mcp/tree/main/examples/plugins/qr-code)
- [Hash Plugin](https://github.com/tuananh/hyper-mcp-hash-plugin)

### Publishing Plugins

Publishing a plugin to an OCI registry is straightforward:

1. Build your WebAssembly plugin
2. Create a Dockerfile:

```dockerfile
FROM scratch
WORKDIR /
COPY --from=builder /workspace/target/wasm32-wasip1/release/your-plugin.wasm /plugin.wasm
```

3. Build and push:
```sh
docker build -t your-registry/plugin-name .
docker push your-registry/plugin-name
```

## License

[Apache 2.0](./LICENSE)
