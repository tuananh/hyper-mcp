# crates-io

A plugin that fetches crate information and latest versions from crates.io.

## What it does

Provides two main functionalities:
1. `crates_io_latest_version`: Fetches the latest version of a crate
2. `crates_io_crate_info`: Fetches detailed information about a crate including description, downloads, repository, documentation, etc.

## Usage

Call with:
```json
{
  "plugins": [
    {
      "name": "crates-io",
      "path": "oci://ghcr.io/tuananh/crates-io-plugin:latest",
      "runtime_config": {
        "allowed_hosts": ["crates.io"]
      }
    }
  ]
}
```

### Example Usage

1. Get latest version of a crate:
```json
{
  "name": "crates_io_latest_version",
  "params": {
    "crate_name": "serde"
  }
}
```

2. Get detailed information about a crate:
```json
{
  "name": "crates_io_crate_info",
  "params": {
    "crate_name": "serde"
  }
}

```

Returns:
- For `crates_io_latest_version`: The latest version number as a string
- For `crates_io_crate_info`: A JSON object containing detailed crate information including:
  - Name
  - Description
  - Latest version
  - Download count
  - Repository URL
  - Documentation URL
  - Homepage URL
  - Keywords
  - Categories
  - License
  - Creation and update timestamps