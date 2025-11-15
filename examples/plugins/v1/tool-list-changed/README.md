# Tool List Changed Plugin

This plugin demonstrates dynamic tool list management in hyper-mcp. It showcases how a plugin can modify its tool list at runtime and notify the MCP server about these changes.

## Features

- **Dynamic Tool Creation**: Starts with a single `add_tool` and dynamically creates new tools
- **Host Function Integration**: Uses the `notify_tool_list_changed` host function to notify the server
- **Atomic Counter**: Thread-safe tool counting using atomic operations

## How It Works

The plugin begins with only one callable tool:

- `add_tool`: When called, this tool creates a new tool named `tool_n` (where n starts at 1 and increments)

After each call to `add_tool`:
1. A new tool `tool_n` is added to the plugin's tool list
2. The plugin calls `notify_tool_list_changed()` to inform the MCP server
3. The server updates its understanding of available tools

## Tools

### Initial Tool

- **add_tool**: Creates a new dynamic tool and notifies the server of the tool list change
  - Takes no parameters
  - Returns a JSON object with the new tool name and current tool count

### Dynamic Tools

- **tool_1, tool_2, tool_3, ...**: Created dynamically when `add_tool` is called
  - Each tool returns information about itself when called
  - Takes no parameters

## Usage Example

1. **Initial state**: Only `add_tool` is available
2. **Call `add_tool`**: Creates `tool_1` and notifies the server
3. **Call `add_tool` again**: Creates `tool_2` and notifies the server
4. **Call `tool_1`**: Returns information about being the first dynamically created tool

## Building

```bash
cd hyper-mcp/examples/plugins/v1/tool-list-changed
cargo build --target wasm32-unknown-unknown --release
```

The compiled WASM file will be available at:
`target/wasm32-unknown-unknown/release/tool_list_changed.wasm`

## Configuration

This plugin requires no additional configuration. It uses atomic operations to maintain thread-safe state across calls.

## Implementation Details

- Uses `AtomicUsize` for thread-safe tool counting
- Calls the `notify_tool_list_changed` host function after each tool addition
- Implements both static (`add_tool`) and dynamic (`tool_n`) tool handling
- Provides JSON responses with relevant information about operations
