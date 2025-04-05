# github

[src](https://github.com/dylibso/mcp.run-servlets/tree/main/servlets/github)

## Usage

```json
{
    "plugins": [
        {
            "name": "github",
            "path": "/home/anh/Code/hyper-mcp/examples/plugins/github/dist/plugin.wasm",
            "runtime_config": {
                "allowed_hosts": [
                    "api.github.com"
                ],
                "env_vars": {
                    "api-key": "ghp_xxxx"
                }
            }
        }
    ]
}
```