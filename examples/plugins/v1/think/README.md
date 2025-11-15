# think

A simple MCP plugin that returns the provided thought string. Useful for agentic reasoning, cache memory, or when you want to "think out loud" in a workflow.

## What it does

Takes a `thought` parameter (string) and simply returns it as the result. No side effects, no logging, no database or network calls.

Read more about the think tool in [this blog post](https://www.anthropic.com/engineering/claude-think-tool).

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

## Example usage with Cursor/Windsurf

Add a new Cursor/Windsurf rule like the following

```
After any context change (viewing new files, running commands, or receiving tool outputs), use the "think" tool to organize your reasoning before responding.

Specifically, always use the think tool when:
- After examining file contents or project structure
- After running terminal commands or analyzing their outputs
- After receiving search results or API responses
- Before making code suggestions or explaining complex concepts
- When transitioning between different parts of a task

When using the think tool:
- List the specific rules or constraints that apply to the current task
- Check if all required information is collected
- Verify that your planned approach is correct
- Break down complex problems into clearly defined steps
- Analyze outputs from other tools thoroughly
- Plan multi-step approaches before executing them

The think tool has been proven to improve performance by up to 54% on complex tasks, especially when working with multiple tools or following detailed policies.
```
