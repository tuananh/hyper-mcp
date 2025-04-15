# qdrant-rag

A plugin that provides RAG (Retrieval Augmented Generation) capabilities using Qdrant vector database and FastEmbed for embeddings.

## What it does

This plugin provides three main functionalities:
1. Generate embeddings for text using FastEmbed
2. Store documents with their embeddings in Qdrant
3. Search for similar documents using either text queries or vector embeddings

## Configuration

The plugin requires the following configuration:

```json
{
  "plugins": [
    {
      "name": "qdrant-rag",
      "path": "oci://ghcr.io/tuananh/qdrant-rag-plugin:latest",
      "runtime_config": {
        "qdrant_url": "http://localhost:6334",
        "embedding_model": "BAAI/bge-small-en-v1.5",
        "allowed_hosts": [
          "localhost:6334"
        ]
      }
    }
  ]
}
```

## Tools

### 1. embed_text
Generates vector embeddings for given text using the configured model.

```json
{
  "text": "Your text here"
}
```

### 2. qdrant_store
Stores a document with its vector embedding in Qdrant. The vector can be provided or will be automatically generated.

```json
{
  "collection_name": "my_documents",
  "text": "Your document text",
  "vector": [0.1, 0.2, ...] // Optional: will be generated if not provided
}
```

### 3. qdrant_find
Finds similar documents using either text query or vector similarity search.

```json
{
  "collection_name": "my_documents",
  "query": "Your search query",  // Either query or vector must be provided
  "vector": [0.1, 0.2, ...],    // Either query or vector must be provided
  "limit": 5                    // Optional, defaults to 5
}
```

## Features
- Automatic collection creation with appropriate vector dimensions
- Lazy loading of embedding model
- Support for both text and vector-based queries
- Configurable similarity search results limit
- Uses cosine similarity for vector matching
- Thread-safe model instance management

## Dependencies
- FastEmbed for text embeddings
- Qdrant for vector storage and similarity search
- Tokio for async runtime
- UUID for document identification