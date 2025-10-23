import { z, type ZodType } from "zod";
import {
  EntityDetails,
  EntityNotFoundError,
  IndexNotInitializedError,
  InvalidParamsError,
  JsonRpcError,
  JsonRpcErrorCode,
  JsonRpcErrorObject,
  JsonRpcId,
  JsonRpcFailureSchema,
  JsonRpcSuccessFactory,
  QueryTimeoutError,
  RebuildIndexParams,
  RebuildIndexResult,
  RebuildIndexResultSchema,
  RetrieveEntityParams,
  RetrieveEntityResult,
  RetrieveEntityResultSchema,
  SearchEntitiesParams,
  SearchEntitiesResult,
  SearchEntitiesResultSchema,
  TraverseGraphParams,
  TraverseGraphResult,
  TraverseGraphResultSchema,
  UnexpectedResponseError,
} from "../types/api";

interface JsonRpcClientOptions {
  headers?: HeadersInit;
  fetchImplementation?: typeof fetch;
  timeoutMs?: number;
  retryDelaysMs?: number[];
  logger?: (event: string, payload?: unknown) => void;
}

export class NetworkError extends Error {
  constructor(message: string, options?: { cause?: unknown }) {
    super(message, options);
    this.name = "NetworkError";
  }
}

export class JSONRPCClient {
  private readonly endpoint: string;
  private readonly headers: HeadersInit;
  private readonly fetchImpl: typeof fetch;
  private readonly timeoutMs: number;
  private readonly retryDelays: number[];
  private readonly logger?: (event: string, payload?: unknown) => void;
  private idCounter = 0;

  constructor(endpoint: string, options: JsonRpcClientOptions = {}) {
    this.endpoint = endpoint.replace(/\/$/, "");
    this.headers = options.headers ?? {};
    this.fetchImpl = options.fetchImplementation ?? globalThis.fetch;
    if (!this.fetchImpl) {
      throw new Error("fetch implementation is required (provide global fetch or custom implementation)");
    }
    this.timeoutMs = options.timeoutMs ?? 30_000;
    this.retryDelays = options.retryDelaysMs ?? [1_000, 2_000, 4_000];
    this.logger = options.logger;
  }

  async call<T>(
    method: string,
    params: Record<string, unknown> = {},
    options: { schema?: ZodType<T>; timeoutMs?: number } = {},
  ): Promise<T> {
    const schema = options.schema ?? (z.unknown() as ZodType<T>);
    return this.performCall(method, params, schema, options.timeoutMs);
  }

  async searchEntities(params: SearchEntitiesParams): Promise<SearchEntitiesResult> {
    const rpcParams = this.compact({
      query: params.query,
      entity_types: params.entityTypes,
      limit: params.limit,
      use_bm25: params.useBm25,
      snippet_mode: params.snippetMode,
    });

    return this.performCall(
      "search_entities",
      rpcParams,
      SearchEntitiesResultSchema,
    );
  }

  async traverseGraph(params: TraverseGraphParams): Promise<TraverseGraphResult> {
    const rpcParams = this.compact({
      start_entities: params.startEntities,
      depth: params.depth,
      relations: params.relations,
      entity_types: params.entityTypes,
      direction: params.direction,
      format: params.format,
    });

    return this.performCall(
      "traverse_graph",
      rpcParams,
      TraverseGraphResultSchema,
    );
  }

  async retrieveEntity(params: RetrieveEntityParams): Promise<RetrieveEntityResult> {
    const rpcParams = this.compact({
      entity_ids: params.entityIds,
      include_context: params.includeContext,
      include_metadata: params.includeMetadata,
    });

    return this.performCall(
      "retrieve_entity",
      rpcParams,
      RetrieveEntityResultSchema,
    );
  }

  async getEntityDetails(
    entityId: string,
    options: Omit<RetrieveEntityParams, "entityIds"> = {},
  ): Promise<EntityDetails> {
    const result = await this.retrieveEntity({
      entityIds: [entityId],
      includeContext: options.includeContext,
      includeMetadata: options.includeMetadata,
    });

    const entity = result.entities[0];
    if (!entity) {
      throw new EntityNotFoundError("Entity not found", { entityId });
    }
    return entity;
  }

  async initializeIndex(params: RebuildIndexParams): Promise<RebuildIndexResult> {
    const rpcParams = this.compact({
      repo_path: params.repoPath,
      languages: params.languages,
      incremental: params.incremental,
      output_path: params.outputPath,
    });

    return this.performCall(
      "rebuild_index",
      rpcParams,
      RebuildIndexResultSchema,
    );
  }

  async rebuildIndex(params: RebuildIndexParams): Promise<RebuildIndexResult> {
    return this.initializeIndex(params);
  }

  private async performCall<T>(
    method: string,
    params: Record<string, unknown>,
    schema: ZodType<T>,
    timeoutOverride?: number,
  ): Promise<T> {
    const id = this.nextId();
    const body = JSON.stringify({
      jsonrpc: "2.0" as const,
      id,
      method,
      params,
    });

    const attempts = this.retryDelays.length + 1;
    for (let attempt = 0; attempt < attempts; attempt += 1) {
      try {
        this.log("request", { method, params, id, attempt });
        const response = await this.executeFetch(body, timeoutOverride);
        const payload = await this.parseJson(response);
        const failure = JsonRpcFailureSchema.safeParse(payload);
        if (failure.success) {
          this.assertMatchingId(failure.data.id, id);
          throw this.mapJsonRpcError(failure.data.error);
        }

        const successSchema = JsonRpcSuccessFactory(schema);
        const success = successSchema.safeParse(payload);
        if (!success.success) {
          throw new UnexpectedResponseError("JSON-RPC response validation failed");
        }

        this.assertMatchingId(success.data.id, id);
        const result = schema.parse(success.data.result);
        this.log("response", { method, result, id, attempt });
        return result;
      } catch (rawError) {
        const error = this.normalizeError(rawError);
        if (!(error instanceof NetworkError)) {
          this.log("error", { method, error, id, attempt });
          throw error;
        }

        const hasRetry = attempt < attempts - 1;
        this.log("networkError", { method, error, id, attempt, willRetry: hasRetry });
        if (!hasRetry) {
          throw error;
        }
        await this.delay(this.retryDelays[attempt]);
      }
    }

    throw new NetworkError("Exceeded maximum retry attempts");
  }

  private async executeFetch(body: string, timeoutOverride?: number): Promise<Response> {
    const controller = new AbortController();
    const timeout = timeoutOverride ?? this.timeoutMs;
    const timer = timeout > 0 ? setTimeout(() => controller.abort(), timeout) : undefined;

    try {
      const response = await this.fetchImpl(this.endpoint, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          ...this.headers,
        },
        body,
        signal: controller.signal,
      });

      if (!response.ok) {
        throw new NetworkError(`HTTP error ${response.status}`, { cause: response.statusText });
      }

      return response;
    } catch (error) {
      if (error instanceof DOMException && error.name === "AbortError") {
        throw new NetworkError("Request timed out", { cause: error });
      }
      if (error instanceof Error) {
        throw new NetworkError(error.message, { cause: error });
      }
      throw new NetworkError("Unknown network error", { cause: error });
    } finally {
      if (timer !== undefined) {
        clearTimeout(timer);
      }
    }
  }

  private async parseJson(response: Response): Promise<unknown> {
    try {
      return await response.json();
    } catch (error) {
      const text = await response.text().catch(() => undefined);
      throw new UnexpectedResponseError(
        `Failed to parse JSON-RPC response: ${(error as Error).message}${text ? ` (body: ${text})` : ""}`,
      );
    }
  }

  private mapJsonRpcError(error: JsonRpcErrorObject): JsonRpcError {
    switch (error.code) {
      case JsonRpcErrorCode.IndexNotFound:
        return new IndexNotInitializedError(error.message, error.data);
      case JsonRpcErrorCode.EntityNotFound:
        return new EntityNotFoundError(error.message, error.data);
      case JsonRpcErrorCode.QueryTimeout:
        return new QueryTimeoutError(error.message, error.data);
      case JsonRpcErrorCode.InvalidParams:
        return new InvalidParamsError(error.message, error.data);
      default:
        return new JsonRpcError(error.message, error.code, error.data);
    }
  }

  private normalizeError(error: unknown): Error {
    if (error instanceof JsonRpcError || error instanceof UnexpectedResponseError || error instanceof NetworkError) {
      return error;
    }

    if (error instanceof Error) {
      return error;
    }

    return new NetworkError("Unknown error", { cause: error });
  }

  private compact(params: Record<string, unknown>): Record<string, unknown> {
    return Object.entries(params).reduce<Record<string, unknown>>((acc, [key, value]) => {
      if (value !== undefined && value !== null) {
        acc[key] = value;
      }
      return acc;
    }, {});
  }

  private assertMatchingId(responseId: JsonRpcId, requestId: number): void {
    if (responseId === null || responseId !== requestId) {
      throw new UnexpectedResponseError(
        `JSON-RPC response id mismatch: expected ${requestId}, received ${String(responseId)}`,
      );
    }
  }

  private nextId(): number {
    this.idCounter += 1;
    return this.idCounter;
  }

  private async delay(ms: number): Promise<void> {
    if (ms <= 0) {
      return;
    }
    await new Promise<void>((resolve) => {
      setTimeout(resolve, ms);
    });
  }

  private log(event: string, payload: unknown): void {
    if (this.logger) {
      this.logger(event, payload);
    }
  }
}
