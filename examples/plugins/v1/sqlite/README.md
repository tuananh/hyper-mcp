# sqlite

A plugin that provide SQLite interactions for `hyper-mcp`.

## Usage

Call with:
```json
{
  "plugins": [
    // {},
    {
        "name": "sqlite",
        "path": "oci://ghcr.io/tuananh/sqlite-plugin",
        "runtime_config": {
          "allowed_paths": ["/tmp"],
          "env_vars": {
            "db_path": "/tmp/memory.db"
          }
        }
      }
  ]
}

```

## How to build

This plugin requires you to have [wasi-sdk](https://github.com/WebAssembly/wasi-sdk) installed.

```sh
export WASI_SDK_PATH=`<wasi-sdk-path>` # in my case, it's /opt/wasi-sdk
export CC_wasm32_wasip1="${WASI_SDK_PATH}/bin/clang --sysroot=${WASI_SDK_PATH}/share/wasi-sysroot"
cargo build --release --target wasm32-wasip1
```

See [Dockerfile](./Dockerfile) for reference.
