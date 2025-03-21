# browser

A example `hyper-mcp` plugin that can perform tasks with browser.

Looks like WASI doesn't support WebSocket yet so it's no go for now.

## Usage

```json
{
  "plugins": [
    // ....
    {
      "name": "browser",
      "path": "/home/anh/Code/hyper-mcp/examples/plugins/browser/target/wasm32-wasip1/release/browser.wasm",
      "runtime_config": {
        "allowed_host": "127.0.0.1:9222",
        "config": {
          "CHROME_CDP_URL": "ws://127.0.0.1:9222"
        }
      }
    }
  ]
}
```