# gomodule

A plugin that fetches Go module information and latest versions from `proxy.golang.org`.

## What it does

Provides two main functionalities:
1. `go_module_latest_version`: Fetches the latest version of multiple Go modules
2. `go_module_info`: Fetches detailed information about multiple Go modules

## Usage

Call with:
```json
{
  "plugins": [
    {
      "name": "gomodule",
      "path": "oci://ghcr.io/tuananh/gomodule-plugin:latest",
      "runtime_config": {
        "allowed_hosts": ["proxy.golang.org"]
      }
    }
  ]
}
```

### Example Usage

1. Get latest version of multiple Go modules:
```json
{
  "name": "go_module_latest_version",
  "params": {
    "module_names": "github.com/spf13/cobra,github.com/gorilla/mux,github.com/gin-gonic/gin"
  }
}
```

2. Get detailed information about multiple Go modules:
```json
{
  "name": "go_module_info",
  "params": {
    "module_names": "github.com/spf13/cobra,github.com/gorilla/mux,github.com/gin-gonic/gin"
  }
}
```

Returns:
- For `go_module_latest_version`: A JSON object mapping module names to their latest version numbers
- For `go_module_info`: An array of JSON objects containing detailed module information for each module, including:
  - Name
  - Latest version
  - Time
  - Version
  - And other metadata from proxy.golang.org
