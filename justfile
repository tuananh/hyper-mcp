hyper_mcp_bin := `realpath ./target/debug/hyper-mcp`

debug:
    npx @modelcontextprotocol/inspector {{hyper_mcp_bin}} --config-file ~/.config/hyper-mcp/config.json

run:
    cargo run

renovate:
    docker run --rm \
        -e RENOVATE_TOKEN=$(gh auth token) \
        -e LOG_LEVEL=info \
        -v "$(pwd)/.github/renovate.json5:/usr/src/app/config.json" \
        renovate/renovate tuananh/hyper-mcp
