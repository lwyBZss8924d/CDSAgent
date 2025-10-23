/**
 * CDSAgent TypeScript Client Library
 *
 * Main entry point for the CDSAgent JSON-RPC client and type definitions.
 *
 * @example
 * ```typescript
 * import { JSONRPCClient, SearchEntitiesParams } from 'cds-agent';
 *
 * const client = new JSONRPCClient('http://localhost:9876/rpc');
 *
 * const params: SearchEntitiesParams = {
 *   query: 'sanitize',
 *   limit: 10,
 *   entityTypes: ['function']
 * };
 *
 * const results = await client.searchEntities(params);
 * console.log(results.entities);
 * ```
 */

// Main client export
export { JSONRPCClient, NetworkError } from "./client/jsonrpc";

// Type definitions and schemas
export {
  // Core types
  type Entity,
  type EntityDetails,
  type EntityType,
  type GraphNode,
  type GraphEdge,
  type RelationType,
  type SnippetMode,
  type Snippet,
  type LineRange,

  // Result types
  type SearchEntitiesResult,
  type TraverseGraphResult,
  type RetrieveEntityResult,
  type RebuildIndexResult,
  type RebuildStats,
  type QueryMetadata,

  // Request parameter types
  type SearchEntitiesParams,
  type TraverseGraphParams,
  type RetrieveEntityParams,
  type RebuildIndexParams,

  // JSON-RPC types
  type JsonRpcId,
  type JsonRpcRequest,
  type JsonRpcResponse,
  type JsonRpcSuccess,
  type JsonRpcFailure,
  type JsonRpcErrorObject,
  JsonRpcErrorCode,

  // Error classes
  JsonRpcError,
  IndexNotInitializedError,
  EntityNotFoundError,
  QueryTimeoutError,
  InvalidParamsError,
  UnexpectedResponseError,

  // Zod schemas (for advanced usage)
  EntitySchema,
  EntityDetailsSchema,
  GraphNodeSchema,
  GraphEdgeSchema,
  SearchEntitiesResultSchema,
  TraverseGraphResultSchema,
  RetrieveEntityResultSchema,
  RebuildIndexResultSchema,
} from "./types/api";

// Configuration utilities
export {
  type CDSAgentConfig,
  loadConfig,
  defaultConfig,
} from "./utils/config";