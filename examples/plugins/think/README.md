# think

A simple MCP plugin that returns the provided thought string. Useful for agentic reasoning, cache memory, or when you want to "think out loud" in a workflow.

## What it does

Takes a `thought` parameter (string) and simply returns it as the result. No side effects, no logging, no database or network calls.

## Usage

Call with:
```json
{
  "plugins": [
    {
      "name": "think",
      "path": "oci://ghcr.io/tuananh/think-plugin:latest"
    }
  ]
}
```

### Example

Tool call:
```json
{
  "name": "think",
  "arguments": { "thought": "I should try a different approach." }
}
```
Returns:
```json
"I should try a different approach."
```
