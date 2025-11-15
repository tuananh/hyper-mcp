# arxiv

A plugin that let you search for papers on arXiv and download them.

## Usage

Call with:
```json
{
  "plugins": [
    // {},
    {
        "name": "arxiv",
        "path": "/home/anh/Code/hyper-mcp/examples/plugins/v1/arxiv/target/wasm32-wasip1/release/plugin.wasm",
        "runtime_config": {
          "allowed_hosts": ["export.arxiv.org", "arxiv.org"],
          "allowed_paths": ["/tmp"]
        }
      }
  ]
}

```
