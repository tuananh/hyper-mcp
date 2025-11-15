# rstime Plugin

A Model Context Protocol (MCP) plugin for working with time and timezone information. The `rstime` plugin provides tools for retrieving the current time in different timezones and parsing RFC2822 formatted time strings.

## Overview

The rstime plugin is a WebAssembly-based MCP plugin written in Rust that exposes time-related functionality through the Model Context Protocol. It allows LLM clients to:

- Get the current time in any timezone
- Parse RFC2822 formatted time strings to Unix timestamps
- Complete timezone names for better user experience

## Features

### Tools

#### `get_time`
Returns the current time in a specified timezone.

**Input:**
- `timezone` (optional, string): The timezone identifier (e.g., `America/New_York`, `Europe/London`, `Asia/Tokyo`). Defaults to `UTC` if not provided.

**Output:**
- `current_time` (string): The current time in RFC2822 format for the specified timezone.

**Example:**
```
Tool: get_time
Input: {"timezone": "America/Los_Angeles"}
Output: "Tue, 15 Jan 2024 10:30:45 -0800"
```

#### `parse_time`
Parses an RFC2822 formatted time string and returns the corresponding Unix timestamp.

**Input:**
- `time` (required, string): The time string in RFC2822 format to parse.

**Output:**
- `timestamp` (integer): The parsed timestamp in seconds since the Unix epoch.

**Example:**
```
Tool: parse_time
Input: {"time": "Tue, 15 Jan 2024 18:30:45 +0000"}
Output: 1705344645
```

### Prompts

#### `get_time_with_timezone`
A prompt that guides the user to get the current time for a specific timezone.

**Arguments:**
- `timezone` (optional, string): The timezone to retrieve time information for. Defaults to `UTC`.

**Response:**
Returns an assistant message suggesting to get the time for the specified timezone.

## Building

### Prerequisites
- Rust 1.70 or later
- Extism toolchain for WASM compilation

### Build Steps

```bash
# Build the WebAssembly plugin
cargo build --release --target wasm32-wasip1

# The compiled WASM file will be available at target/wasm32-wasip1/release/plugin.wasm
```

Alternatively, you can use the provided `prepare.sh` script:

```bash
./prepare.sh
```

This will create the compiled `rstime.wasm` file.

## Docker Support

A Dockerfile is included for containerized deployment. Build it with:

```bash
docker build -t rstime:latest .
```

## Testing

The plugin includes comprehensive test coverage. Run tests with, changing the target to your architecture:

```bash
cargo test --lib --target x86_64-apple-darwin
```

### Test Coverage

The test suite covers:
- Getting time in UTC and various timezones
- Handling invalid timezone names
- Parsing valid and invalid RFC2822 time strings
- Tool listing and metadata
- Prompt retrieval and listing
- Resource operations
- Error handling and edge cases

## Supported Timezones

The plugin supports all timezones from the IANA timezone database through the `chrono-tz` crate. Some common examples include:

- `UTC` - Coordinated Universal Time
- `America/New_York` - Eastern Time
- `America/Chicago` - Central Time
- `America/Denver` - Mountain Time
- `America/Los_Angeles` - Pacific Time
- `Europe/London` - Greenwich Mean Time
- `Europe/Paris` - Central European Time
- `Asia/Tokyo` - Japan Standard Time
- `Asia/Shanghai` - China Standard Time
- `Australia/Sydney` - Australian Eastern Time

For a complete list, refer to the IANA timezone database.

## Dependencies

- **chrono** - Date and time handling
- **chrono-tz** - Timezone support
- **extism-pdk** - Extism Plugin Development Kit for MCP
- **serde** - Serialization/deserialization
- **serde_json** - JSON handling
- **anyhow** - Error handling
- **base64** - Base64 encoding/decoding

## Usage Example

When integrated with an MCP-compatible client, you can use the plugin like this:

```
User: What time is it in Tokyo?

Client calls: get_time tool with {"timezone": "Asia/Tokyo"}
Plugin returns: Current time in RFC2822 format for Asia/Tokyo

Client: The current time in Tokyo is [result]
```

## Architecture

The plugin follows the Extism PDK architecture:

- **lib.rs**: Main plugin implementation with tool and prompt logic
- **pdk**: PDK-specific types and exported functions for MCP communication
- **Cargo.toml**: Rust dependencies and build configuration

The plugin compiles to WebAssembly and implements the MCP protocol through Extism's foreign function interface.

## Error Handling

The plugin gracefully handles common errors:

- **Invalid timezone**: Returns a descriptive error message
- **Missing required arguments**: Provides helpful feedback about required parameters
- **Invalid time format**: Reports parsing errors with context
- **Unknown tools/prompts**: Returns appropriate error responses

## Contributing

When contributing to this plugin:

1. Maintain the existing code style
2. Add tests for new functionality
3. Update this README with any new features
4. Ensure all tests pass before submitting

## License

This plugin is part of the hyper-mcp project. See the main project repository for license information.
