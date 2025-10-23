# CDS-Agent TypeScript Client

TypeScript client library and Claude Agent SDK integration for the CDSAgent code retrieval system.

## Installation

```bash
# Install dependencies
bun install

# Run tests
bun test

# Type checking
bun run typecheck
```

## Usage

### Basic Client Usage

```typescript
import { JSONRPCClient } from 'cds-agent';

// Initialize the client
const client = new JSONRPCClient('http://localhost:9876/rpc');

// Search for entities
const searchResults = await client.searchEntities({
  query: 'sanitize input',
  entityTypes: ['function'],
  limit: 10,
  snippetMode: 'preview'
});

console.log(`Found ${searchResults.total_count} entities`);
searchResults.entities.forEach(entity => {
  console.log(`- ${entity.name} in ${entity.file_path}`);
});

// Traverse the code graph
const graphResults = await client.traverseGraph({
  startEntities: ['entity_id_123'],
  depth: 2,
  relations: ['invoke', 'inherit'],
  direction: 'forward'
});

console.log(`Graph contains ${graphResults.metadata.total_nodes} nodes`);

// Retrieve entity details
const details = await client.retrieveEntity({
  entityIds: ['entity_id_123'],
  includeContext: 10,
  includeMetadata: true
});

details.entities.forEach(entity => {
  console.log(`${entity.name}: ${entity.code}`);
});

// Rebuild index for a repository
const rebuildResult = await client.rebuildIndex({
  repoPath: '/path/to/repository',
  languages: ['python', 'typescript'],
  incremental: false
});

console.log(`Indexed ${rebuildResult.stats.files_indexed} files`);
```

### Configuration

Configuration can be set via environment variables. See `.env.example` for all options:

```bash
# Copy example configuration
cp .env.example .env

# Edit configuration
vim .env
```

Key configuration options:

- `CDS_INDEX_SERVICE_URL`: JSON-RPC service endpoint (default: `http://localhost:9876/rpc`)
- `CDS_REQUEST_TIMEOUT_MS`: Request timeout in milliseconds (default: 30000)
- `CDS_RETRY_DELAYS_MS`: Comma-separated retry delays (default: `1000,2000,4000`)
- `CDS_LOG_LEVEL`: Log level (`debug`, `info`, `warn`, `error`, `silent`)

### Error Handling

The client provides typed error classes for different failure scenarios:

```typescript
import {
  IndexNotInitializedError,
  EntityNotFoundError,
  QueryTimeoutError,
  NetworkError
} from 'cds-agent';

try {
  const results = await client.searchEntities({ query: 'test' });
} catch (error) {
  if (error instanceof IndexNotInitializedError) {
    console.error('Index not initialized. Run rebuild_index first.');
  } else if (error instanceof QueryTimeoutError) {
    console.error('Search query timed out. Try a more specific query.');
  } else if (error instanceof NetworkError) {
    console.error('Network error:', error.message);
  }
}
```

### Type Definitions

All API types are fully typed with TypeScript and runtime validation via Zod:

```typescript
import type {
  SearchEntitiesParams,
  SearchEntitiesResult,
  Entity,
  EntityType,
  SnippetMode
} from 'cds-agent';

// Type-safe parameter construction
const params: SearchEntitiesParams = {
  query: 'process',
  entityTypes: ['function', 'class'],
  limit: 20,
  useBm25: true,
  snippetMode: 'preview'
};

// Results are fully typed
const result: SearchEntitiesResult = await client.searchEntities(params);
result.entities.forEach((entity: Entity) => {
  console.log(entity.name); // TypeScript knows all properties
});
```

## Development

### Running Tests

```bash
# Run all tests
bun test

# Watch mode
bun test --watch

# Run specific test file
bun test tests/jsonrpc-client.test.ts
```

### Project Structure

```tree
cds-agent/
├── src/
│   ├── client/
│   │   └── jsonrpc.ts       # JSON-RPC client implementation
│   ├── types/
│   │   └── api.ts           # Type definitions and Zod schemas
│   ├── utils/
│   │   └── config.ts        # Configuration management
│   ├── index.ts             # Main exports
│   └── main.ts              # Agent entrypoint
├── tests/
│   └── jsonrpc-client.test.ts  # Unit tests
├── package.json
├── tsconfig.json
├── .env.example
└── README.md
```

## API Methods

### `searchEntities(params)`

Search for code entities using hierarchical name index and BM25 content search.

**Parameters:**

- `query` (string, required): Search keywords
- `entityTypes` (array): Filter by entity types (`directory`, `file`, `class`, `function`)
- `limit` (number): Maximum results (default: 10, max: 100)
- `useBm25` (boolean): Enable BM25 content search (default: true)
- `snippetMode` (string): Code detail level (`fold`, `preview`, `full`)

### `traverseGraph(params)`

Traverse the code dependency graph using BFS from starting entities.

**Parameters:**

- `startEntities` (array, required): Entity IDs to start from
- `depth` (number): Maximum traversal depth (default: 1, max: 10)
- `relations` (array): Filter by edge types (`contain`, `import`, `invoke`, `inherit`)
- `entityTypes` (array): Filter by node types
- `direction` (string): Traversal direction (`forward`, `backward`, `bidirectional`)
- `format` (string): Output format (`graph`, `tree`)

### `retrieveEntity(params)`

Retrieve full entity details with optional context lines.

**Parameters:**

- `entityIds` (array, required): Entity IDs to retrieve
- `includeContext` (number): Lines of context before/after (max: 50)
- `includeMetadata` (boolean): Include AST metadata

### `rebuildIndex(params)`

Rebuild the code graph and BM25 indices for a repository.

**Parameters:**

- `repoPath` (string, required): Path to repository root
- `languages` (array): Languages to parse (`python`, `typescript`, `javascript`, `rust`)
- `incremental` (boolean): Update only changed files
- `outputPath` (string): Custom index directory

## Integration with Claude Agent SDK

The client is designed to integrate with the Claude Agent SDK for LLM-powered code exploration:

```typescript
// main.ts shows the integration point
import { JSONRPCClient } from 'cds-agent';

const jsonRpcClient = new JSONRPCClient(serviceUrl, {
  logger: (event, payload) => {
    // Integrated logging with Claude Agent SDK
    console.debug(`[jsonrpc:${event}]`, payload);
  }
});

// TODO: Register with Claude Agent SDK hooks
// Once the SDK is initialized, the client will be available
// for tool handlers to use for code retrieval
```

## License

MIT - See LICENSE file for details
