# qdrant

A plugin that provides vector similarity search capabilities using Qdrant vector database.

## What it does

This plugin provides three main functionalities:
1. Create collections with configurable vector dimensions
2. Store documents with their vector embeddings in Qdrant
3. Search for similar documents using vector embeddings

## Configuration

The plugin requires the following configuration:

```json
{
  "plugins": [
    {
      "name": "qdrant",
      "path": "oci://ghcr.io/tuananh/qdrant-plugin:latest",
      "runtime_config": {
        "QDRANT_URL": "http://localhost:6334",
        "allowed_hosts": [
          "localhost:6333"
        ],
        "env_vars": {
          "QDRANT_URL": "http://localhost:6333"
        }
      }
    }
  ]
}
```

## Tools

### 1. qdrant_create_collection

Creates a new collection in Qdrant with specified vector dimensions.

```json
{
  "collection_name": "my_documents",
  "vector_size": 384  // Optional, defaults to 384
}
```

### 2. qdrant_store

Stores a document with its vector embedding in Qdrant.

```json
{
  "collection_name": "my_documents",
  "text": "Your document text",
  "vector": [0.1, 0.2, ...] // Vector dimensions must match collection's vector_size
}
```

### 3. qdrant_find

Finds similar documents using vector similarity search.

```json
{
  "collection_name": "my_documents",
  "vector": [0.1, 0.2, ...],    // Vector dimensions must match collection's vector_size
  "limit": 5                    // Optional, defaults to 5
}
```

## Features

- Configurable vector dimensions per collection
- Support for vector-based queries
- Configurable similarity search results limit
- Uses cosine similarity for vector matching
- Thread-safe operations

## Dependencies

- Qdrant for vector storage and similarity search
- UUID for document identification
