hyper_mcp_bin := `realpath ./target/debug/hyper-mcp`

debug:
    npx @modelcontextprotocol/inspector {{hyper_mcp_bin}} --config-file ~/.config/hyper-mcp/config.json

run:
    cargo run
