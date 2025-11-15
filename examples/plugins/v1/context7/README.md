# Context7 API Tools Plugin

This plugin provides tools to interact with the Context7 API, allowing for resolving library IDs and fetching documentation.

## Usage

```
{
  "plugins": [
    {
      "name": "context7",
      "path": "oci://ghcr.io/tuananh/context7-plugin:nightly",
      "runtime_config": {
        "allowed_hosts": ["context7.com"]
      }
    }
  ]
}
```

## Tools

### 1. `c7_resolve_library_id`

**Description:** Resolves a package name to a Context7-compatible library ID and returns a list of matching libraries. You MUST call this function before 'c7_get_library_docs' to obtain a valid Context7-compatible library ID. When selecting the best match, consider: - Name similarity to the query - Description relevance - Code Snippet count (documentation coverage) - GitHub Stars (popularity) Return the selected library ID and explain your choice. If there are multiple good matches, mention this but proceed with the most relevant one.

**Input Schema:**
An object with the following properties:
- `library_name` (string, required): The general name of the library (e.g., 'React', 'upstash/redis').

**Example Input:**
```json
{
  "library_name": "upstash/redis"
}
```

**Output:**
A JSON string containing the resolved Context7 compatible library ID.

**Example Output:**
```json
{
  "context7_compatible_library_id": "upstash_redis_id"
}
```

### 2. `c7_get_library_docs`

**Description:** Fetches up-to-date documentation for a library. You must call 'c7_resolve_library_id' first to obtain the exact Context7-compatible library ID required to use this tool.

**Input Schema:**
An object with the following properties:
- `context7_compatible_library_id` (string, required): The Context7-compatible ID for the library.
- `topic` (string, optional): Focus the docs on a specific topic (e.g., 'routing', 'hooks').
- `tokens` (integer, optional): Max number of tokens for the documentation (default: 10000).

**Example Input:**
```json
{
  "context7_compatible_library_id": "upstash_redis_id",
  "topic": "data_types",
  "tokens": 5000
}
```

**Output:**

The fetched documentation in text format.
