# memory

A plugin that let you save & retrieve memory, backed by SQLite.

## Usage

Call with:
```json
{
  "plugins": [
    // {},
    {
        "name": "memory",
        "path": "/home/anh/Code/hyper-mcp/examples/plugins/v1/memory/target/wasm32-wasip1/release/plugin.wasm",
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
