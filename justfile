hyper_mcp_bin := `realpath ./target/debug/hyper-mcp`

ping:
    echo '{ "jsonrpc": "2.0", "id": 1, "method": "ping" }' | {{hyper_mcp_bin}}

prompts-list:
    echo '{ "jsonrpc": "2.0", "id": 1, "method": "prompts/list" }' | {{hyper_mcp_bin}}

prompt-get:
    echo '{ "jsonrpc": "2.0", "id": 1, "method": "prompts/get", "params": {"name":"current_time","arguments": {"city": "hangzhou"} } }' | {{hyper_mcp_bin}}

tools-list:
    echo '{ "jsonrpc": "2.0", "id": 1, "method": "tools/list" }' | {{hyper_mcp_bin}}

resources-list:
    echo '{ "jsonrpc": "2.0", "id": 1, "method": "resources/list" }' | {{hyper_mcp_bin}}

qr:
    echo '{ "jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": {"name": "qr-code", "arguments": { "data": "hello" } } }' | {{hyper_mcp_bin}}

time:
    echo '{ "jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": { "name": "time", "arguments": { "name": "get_time_utc" } } }' | {{hyper_mcp_bin}}

ip:
    echo '{ "jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": { "name": "myip", "arguments": { "foo": "bar" } } }' | {{hyper_mcp_bin}}

crypto-price:
    RUST_LOG=info echo '{ "jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": { "name": "crypto-price", "arguments": { "symbol": "ethereum" } } }' | {{hyper_mcp_bin}}

write-file:
    RUST_LOG=info echo '{ "jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": { "name": "fs", "arguments": { "operation": "write_file", "path": "/tmp/test.txt", "content": "Hello, world!" } } }' | {{hyper_mcp_bin}}

list-dir:
    RUST_LOG=info echo '{ "jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": { "name": "fs", "arguments": { "operation": "list_dir", "path": "/tmp" } } }' | {{hyper_mcp_bin}}

debug:
    npx @modelcontextprotocol/inspector {{hyper_mcp_bin}} --config-file ~/.config/hyper-mcp/config.json

run:
    cargo run
