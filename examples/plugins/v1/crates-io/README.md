# crates-io

A plugin that fetches crate information and latest versions from crates.io.

## What it does

Provides two main functionalities:
1. `crates_io_latest_version`: Fetches the latest version of multiple crates
2. `crates_io_crate_info`: Fetches detailed information about multiple crates including description, downloads, repository, documentation, etc.

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

1. Get latest version of multiple crates:
```json
{
  "name": "crates_io_latest_version",
  "params": {
    "crate_names": "serde,tokio,clap"
  }
}
```

2. Get detailed information about multiple crates:
```json
{
  "name": "crates_io_crate_info",
  "params": {
    "crate_names": "serde,tokio,clap"
  }
}
```

Returns:
- For `crates_io_latest_version`: A JSON object mapping crate names to their latest version numbers
- For `crates_io_crate_info`: An array of JSON objects containing detailed crate information for each crate, including:
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
