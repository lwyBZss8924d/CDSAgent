import { z } from "zod";

/**
 * Typed bindings for the CDSAgent JSON-RPC API (schema v0.1.0).
 * The zod schemas provide runtime validation for responses coming from the service.
 */

export const EntityTypeSchema = z.enum(["directory", "file", "class", "function"]);
export type EntityType = z.infer<typeof EntityTypeSchema>;

export const RelationTypeSchema = z.enum(["contain", "import", "invoke", "inherit"]);
export type RelationType = z.infer<typeof RelationTypeSchema>;

export const SnippetModeSchema = z.enum(["fold", "preview", "full"]);
export type SnippetMode = z.infer<typeof SnippetModeSchema>;

export const LineRangeSchema = z.tuple([z.number().int().min(1), z.number().int().min(1)]);
export type LineRange = z.infer<typeof LineRangeSchema>;

export const SnippetSchema = z.object({
  fold: z.string(),
  preview: z.string().optional(),
  full: z.string().optional(),
});
export type Snippet = z.infer<typeof SnippetSchema>;

export const EntitySchema = z.object({
  id: z.string(),
  name: z.string(),
  entity_type: EntityTypeSchema,
  file_path: z.string(),
  line_range: LineRangeSchema,
  score: z.number().min(0).max(1),
  snippet: SnippetSchema.optional(),
});
export type Entity = z.infer<typeof EntitySchema>;

export const GraphNodeSchema = z.object({
  id: z.string(),
  name: z.string(),
  entity_type: EntityTypeSchema,
  file_path: z.string(),
  line_range: LineRangeSchema.optional(),
  depth: z.number().int().min(0),
});
export type GraphNode = z.infer<typeof GraphNodeSchema>;

export const GraphEdgeSchema = z.object({
  source: z.string(),
  target: z.string(),
  relation: RelationTypeSchema,
});
export type GraphEdge = z.infer<typeof GraphEdgeSchema>;

export const QueryMetadataSchema = z.object({
  used_upper_index: z.boolean(),
  used_bm25: z.boolean(),
  execution_time_ms: z.number().min(0),
});
export type QueryMetadata = z.infer<typeof QueryMetadataSchema>;

export const SearchEntitiesResultSchema = z.object({
  entities: z.array(EntitySchema),
  total_count: z.number().int().min(0),
  query_metadata: QueryMetadataSchema,
});
export type SearchEntitiesResult = z.infer<typeof SearchEntitiesResultSchema>;

export const TraverseGraphResultSchema = z.object({
  subgraph: z.object({
    nodes: z.array(GraphNodeSchema),
    edges: z.array(GraphEdgeSchema),
  }),
  metadata: z.object({
    total_nodes: z.number().int().min(0),
    total_edges: z.number().int().min(0),
    max_depth_reached: z.number().int().min(0),
    execution_time_ms: z.number().min(0),
  }),
});
export type TraverseGraphResult = z.infer<typeof TraverseGraphResultSchema>;

export const EntityDetailsSchema = z.object({
  id: z.string(),
  name: z.string(),
  entity_type: EntityTypeSchema,
  file_path: z.string(),
  line_range: LineRangeSchema,
  code: z.string(),
  context_before: z.string().optional(),
  context_after: z.string().optional(),
  metadata: z
    .object({
      parameters: z.array(z.string()).optional(),
      return_type: z.string().optional(),
      docstring: z.string().optional(),
      decorators: z.array(z.string()).optional(),
      parent_class: z.string().optional(),
    })
    .optional(),
});
export type EntityDetails = z.infer<typeof EntityDetailsSchema>;

export const RetrieveEntityResultSchema = z.object({
  entities: z.array(EntityDetailsSchema),
});
export type RetrieveEntityResult = z.infer<typeof RetrieveEntityResultSchema>;

export const RebuildStatsSchema = z.object({
  files_indexed: z.number().int().min(0),
  entities_found: z.object({
    directories: z.number().int().min(0),
    files: z.number().int().min(0),
    classes: z.number().int().min(0),
    functions: z.number().int().min(0),
  }),
  edges_created: z.object({
    contain: z.number().int().min(0),
    import: z.number().int().min(0),
    invoke: z.number().int().min(0),
    inherit: z.number().int().min(0),
  }),
  build_time_ms: z.number().min(0),
});
export type RebuildStats = z.infer<typeof RebuildStatsSchema>;

export const RebuildIndexResultSchema = z.object({
  success: z.boolean(),
  stats: RebuildStatsSchema,
  errors: z
    .array(
      z.object({
        file_path: z.string(),
        error: z.string(),
        line: z.number().int().min(0).optional(),
      })
    )
    .optional(),
});
export type RebuildIndexResult = z.infer<typeof RebuildIndexResultSchema>;

// Client-facing parameter types (camelCase for developer ergonomics).
export interface SearchEntitiesParams {
  query: string;
  entityTypes?: EntityType[];
  limit?: number;
  useBm25?: boolean;
  snippetMode?: SnippetMode;
}

export interface TraverseGraphParams {
  startEntities: string[];
  depth?: number;
  relations?: RelationType[];
  entityTypes?: EntityType[];
  direction?: "forward" | "backward" | "bidirectional";
  format?: "graph" | "tree";
}

export interface RetrieveEntityParams {
  entityIds: string[];
  includeContext?: number;
  includeMetadata?: boolean;
}

export interface RebuildIndexParams {
  repoPath: string;
  languages?: Array<"python" | "typescript" | "javascript" | "rust">;
  incremental?: boolean;
  outputPath?: string;
}

export type JsonRpcId = string | number | null;

export interface JsonRpcRequest<TParams> {
  jsonrpc: "2.0";
  id: JsonRpcId;
  method: string;
  params: TParams;
}

export const JsonRpcErrorObjectSchema = z.object({
  code: z.number(),
  message: z.string(),
  data: z.unknown().optional(),
});
export type JsonRpcErrorObject = z.infer<typeof JsonRpcErrorObjectSchema>;

export const JsonRpcSuccessFactory = <T extends z.ZodTypeAny>(resultSchema: T) =>
  z.object({
    jsonrpc: z.literal("2.0"),
    id: z.union([z.string(), z.number(), z.null()]),
    result: resultSchema,
  });

export const JsonRpcFailureSchema = z.object({
  jsonrpc: z.literal("2.0"),
  id: z.union([z.string(), z.number(), z.null()]),
  error: JsonRpcErrorObjectSchema,
});

export type JsonRpcSuccess<T> = {
  jsonrpc: "2.0";
  id: JsonRpcId;
  result: T;
};

export type JsonRpcFailure = z.infer<typeof JsonRpcFailureSchema>;

export type JsonRpcResponse<T> = JsonRpcSuccess<T> | JsonRpcFailure;

export enum JsonRpcErrorCode {
  ParseError = -32700,
  InvalidRequest = -32600,
  MethodNotFound = -32601,
  InvalidParams = -32602,
  InternalError = -32603,
  IndexNotFound = -32001,
  EntityNotFound = -32002,
  ParseFailure = -32003,
  QueryTimeout = -32004,
}

export class JsonRpcError extends Error {
  readonly code: JsonRpcErrorCode | number;
  readonly data?: unknown;

  constructor(message: string, code: JsonRpcErrorCode | number, data?: unknown) {
    super(message);
    this.name = "JsonRpcError";
    this.code = code;
    this.data = data;
  }
}

export class IndexNotInitializedError extends JsonRpcError {
  constructor(message = "Index not initialized", data?: unknown) {
    super(message, JsonRpcErrorCode.IndexNotFound, data);
    this.name = "IndexNotInitializedError";
  }
}

export class EntityNotFoundError extends JsonRpcError {
  constructor(message = "Entity not found", data?: unknown) {
    super(message, JsonRpcErrorCode.EntityNotFound, data);
    this.name = "EntityNotFoundError";
  }
}

export class QueryTimeoutError extends JsonRpcError {
  constructor(message = "Query timed out", data?: unknown) {
    super(message, JsonRpcErrorCode.QueryTimeout, data);
    this.name = "QueryTimeoutError";
  }
}

export class InvalidParamsError extends JsonRpcError {
  constructor(message = "Invalid parameters", data?: unknown) {
    super(message, JsonRpcErrorCode.InvalidParams, data);
    this.name = "InvalidParamsError";
  }
}

export class UnexpectedResponseError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "UnexpectedResponseError";
  }
}
