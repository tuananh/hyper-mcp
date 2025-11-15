# fetch

src: https://github.com/dylibso/mcp.run-servlets/tree/main/servlets/fetch


A servlet that fetches web pages and converts them to markdown.

## What it does

Takes a URL, fetches the page content, strips out scripts and styles, and converts the HTML to markdown format.

## Usage

Call with:
```json
{
  "plugins": [
    // {},
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

Returns the page content converted to markdown format.
