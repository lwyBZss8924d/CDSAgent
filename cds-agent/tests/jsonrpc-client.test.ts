import { describe, expect, test } from "bun:test";
import {
  EntityNotFoundError,
  IndexNotInitializedError,
  QueryTimeoutError,
  UnexpectedResponseError,
} from "../src/types/api";
import { JSONRPCClient, NetworkError } from "../src/client/jsonrpc";

const createJsonResponse = (body: unknown, init?: ResponseInit): Response =>
  new Response(JSON.stringify(body), {
    status: init?.status ?? 200,
    headers: {
      "Content-Type": "application/json",
      ...(init?.headers ?? {}),
    },
  });

describe("JSONRPCClient", () => {
  test("searchEntities returns typed results", async () => {
    let receivedBody: string | undefined;
    const fetchMock: typeof fetch = async (_input, init) => {
      receivedBody = init?.body?.toString();
      return createJsonResponse({
        jsonrpc: "2.0",
        id: 1,
        result: {
          entities: [
            {
              id: "e1",
              name: "sanitize",
              entity_type: "function",
              file_path: "src/app.ts",
              line_range: [10, 42],
              score: 0.9,
              snippet: { fold: "def sanitize" },
            },
          ],
          total_count: 1,
          query_metadata: {
            used_upper_index: true,
            used_bm25: false,
            execution_time_ms: 10,
          },
        },
      });
    };

    const client = new JSONRPCClient("http://localhost:9876/rpc", {
      fetchImplementation: fetchMock,
      retryDelaysMs: [],
    });

    const result = await client.searchEntities({ query: "sanitize" });
    expect(result.entities).toHaveLength(1);
    expect(result.entities[0].name).toBe("sanitize");
    expect(result.query_metadata.used_bm25).toBe(false);

    const parsedBody = JSON.parse(receivedBody ?? "{}") as { method?: string };
    expect(parsedBody.method).toBe("search_entities");
  });

  test("maps JSON-RPC errors to typed exceptions", async () => {
    const fetchMock: typeof fetch = async () =>
      createJsonResponse({
        jsonrpc: "2.0",
        id: 1,
        error: {
          code: -32001,
          message: "Index not found",
          data: { index_path: "/tmp/index" },
        },
      });

    const client = new JSONRPCClient("http://localhost:9876/rpc", {
      fetchImplementation: fetchMock,
      retryDelaysMs: [],
    });

    await expect(client.searchEntities({ query: "sanitize" })).rejects.toBeInstanceOf(
      IndexNotInitializedError,
    );
  });

  test("retries network failures with exponential backoff", async () => {
    let attempts = 0;
    const responses = [
      () => {
        attempts += 1;
        throw new NetworkError("connection refused");
      },
      () => {
        attempts += 1;
        return createJsonResponse({
          jsonrpc: "2.0",
          id: 1,
          result: {
            entities: [],
            total_count: 0,
            query_metadata: {
              used_upper_index: false,
              used_bm25: false,
              execution_time_ms: 1,
            },
          },
        });
      },
    ];

    const fetchMock: typeof fetch = async () => {
      const responder = responses.shift();
      if (!responder) {
        throw new Error("unexpected extra retry");
      }
      const result = responder();
      if (result instanceof Response) {
        return result;
      }
      throw result;
    };

    const client = new JSONRPCClient("http://localhost:9876/rpc", {
      fetchImplementation: fetchMock,
      retryDelaysMs: [0, 0, 0],
    });

    const result = await client.searchEntities({ query: "sanitize" });
    expect(result.total_count).toBe(0);
    expect(attempts).toBe(2);
  });

  test("getEntityDetails throws when entity missing", async () => {
    const fetchMock: typeof fetch = async () =>
      createJsonResponse({
        jsonrpc: "2.0",
        id: 1,
        result: {
          entities: [],
        },
      });

    const client = new JSONRPCClient("http://localhost:9876/rpc", {
      fetchImplementation: fetchMock,
      retryDelaysMs: [],
    });

    await expect(client.getEntityDetails("missing"))
      .rejects.toBeInstanceOf(EntityNotFoundError);
  });

  test("propagates network timeout errors", async () => {
    const fetchMock: typeof fetch = async () =>
      new Promise<Response>((resolve, reject) => {
        setTimeout(() => reject(new DOMException("The operation timed out.", "AbortError")), 1);
      });

    const client = new JSONRPCClient("http://localhost:9876/rpc", {
      fetchImplementation: fetchMock,
      retryDelaysMs: [],
      timeoutMs: 5,
    });

    await expect(client.searchEntities({ query: "sanitize" })).rejects.toBeInstanceOf(NetworkError);
  });

  test("maps query timeout error code", async () => {
    const fetchMock: typeof fetch = async () =>
      createJsonResponse({
        jsonrpc: "2.0",
        id: 1,
        error: {
          code: -32004,
          message: "Query timeout",
        },
      });

    const client = new JSONRPCClient("http://localhost:9876/rpc", {
      fetchImplementation: fetchMock,
      retryDelaysMs: [],
    });

    await expect(client.searchEntities({ query: "sanitize" })).rejects.toBeInstanceOf(QueryTimeoutError);
  });

  test("throws on invalid JSON response", async () => {
    const fetchMock: typeof fetch = async () =>
      new Response("not-json", {
        status: 200,
        headers: { "Content-Type": "application/json" },
      });

    const client = new JSONRPCClient("http://localhost:9876/rpc", {
      fetchImplementation: fetchMock,
      retryDelaysMs: [],
    });

    await expect(client.searchEntities({ query: "sanitize" })).rejects.toBeInstanceOf(
      UnexpectedResponseError,
    );
  });

  test("initializeIndex forwards parameters", async () => {
    let receivedBody: string | undefined;
    const fetchMock: typeof fetch = async (_input, init) => {
      receivedBody = init?.body?.toString();
      return createJsonResponse({
        jsonrpc: "2.0",
        id: 1,
        result: {
          success: true,
          stats: {
            files_indexed: 1,
            entities_found: {
              directories: 0,
              files: 1,
              classes: 0,
              functions: 1,
            },
            edges_created: {
              contain: 0,
              import: 0,
              invoke: 0,
              inherit: 0,
            },
            build_time_ms: 123,
          },
        },
      });
    };

    const client = new JSONRPCClient("http://localhost:9876/rpc", {
      fetchImplementation: fetchMock,
      retryDelaysMs: [],
    });

    const result = await client.initializeIndex({
      repoPath: "/tmp/repo",
      languages: ["python"],
      incremental: true,
      outputPath: "/tmp/out",
    });

    const body = JSON.parse(receivedBody ?? "{}") as Record<string, unknown>;
    expect(body.method).toBe("rebuild_index");
    expect(body.params).toEqual({
      repo_path: "/tmp/repo",
      languages: ["python"],
      incremental: true,
      output_path: "/tmp/out",
    });
    expect(result.success).toBe(true);
  });

  test("traverseGraph handles complex graph structures", async () => {
    const fetchMock: typeof fetch = async () =>
      createJsonResponse({
        jsonrpc: "2.0",
        id: 1,
        result: {
          subgraph: {
            nodes: [
              {
                id: "node1",
                name: "MyClass",
                entity_type: "class",
                file_path: "src/model.py",
                line_range: [10, 50],
                depth: 0,
              },
              {
                id: "node2",
                name: "BaseClass",
                entity_type: "class",
                file_path: "src/base.py",
                line_range: [5, 30],
                depth: 1,
              },
            ],
            edges: [
              {
                source: "node1",
                target: "node2",
                relation: "inherit",
              },
            ],
          },
          metadata: {
            total_nodes: 2,
            total_edges: 1,
            max_depth_reached: 1,
            execution_time_ms: 25,
          },
        },
      });

    const client = new JSONRPCClient("http://localhost:9876/rpc", {
      fetchImplementation: fetchMock,
      retryDelaysMs: [],
    });

    const result = await client.traverseGraph({
      startEntities: ["node1"],
      depth: 2,
      relations: ["inherit"],
      direction: "forward",
    });

    expect(result.subgraph.nodes).toHaveLength(2);
    expect(result.subgraph.edges).toHaveLength(1);
    expect(result.metadata.max_depth_reached).toBe(1);
  });

  test("retrieveEntity handles multiple entities with context", async () => {
    const fetchMock: typeof fetch = async () =>
      createJsonResponse({
        jsonrpc: "2.0",
        id: 1,
        result: {
          entities: [
            {
              id: "entity1",
              name: "process_data",
              entity_type: "function",
              file_path: "src/processor.py",
              line_range: [20, 45],
              code: "def process_data(data):\n    # Process the data\n    return data",
              context_before: "# Helper functions\n",
              context_after: "\n# Main logic",
              metadata: {
                parameters: ["data"],
                return_type: "Any",
                docstring: "Process input data",
              },
            },
          ],
        },
      });

    const client = new JSONRPCClient("http://localhost:9876/rpc", {
      fetchImplementation: fetchMock,
      retryDelaysMs: [],
    });

    const result = await client.retrieveEntity({
      entityIds: ["entity1"],
      includeContext: 5,
      includeMetadata: true,
    });

    expect(result.entities).toHaveLength(1);
    const entity = result.entities[0];
    expect(entity.name).toBe("process_data");
    expect(entity.metadata?.parameters).toEqual(["data"]);
    expect(entity.context_before).toBeDefined();
  });

  test("handles different snippet modes in search results", async () => {
    const fetchMock: typeof fetch = async (_input, init) => {
      const body = JSON.parse((init?.body ?? "{}").toString()) as {
        params?: { snippet_mode?: string };
      };
      const snippetMode = body.params?.snippet_mode;

      const snippet: Record<string, string> = { fold: "def sanitize..." };
      if (snippetMode === "preview" || snippetMode === "full") {
        snippet.preview = "def sanitize(data):\n    # Clean data\n    return clean";
      }
      if (snippetMode === "full") {
        snippet.full = "def sanitize(data):\n    # Full implementation\n    cleaned = data.strip()\n    return cleaned";
      }

      return createJsonResponse({
        jsonrpc: "2.0",
        id: 1,
        result: {
          entities: [
            {
              id: "e1",
              name: "sanitize",
              entity_type: "function",
              file_path: "utils.py",
              line_range: [1, 10],
              score: 1.0,
              snippet,
            },
          ],
          total_count: 1,
          query_metadata: {
            used_upper_index: true,
            used_bm25: false,
            execution_time_ms: 5,
          },
        },
      });
    };

    const client = new JSONRPCClient("http://localhost:9876/rpc", {
      fetchImplementation: fetchMock,
      retryDelaysMs: [],
    });

    // Test fold mode
    const foldResult = await client.searchEntities({
      query: "sanitize",
      snippetMode: "fold",
    });
    expect(foldResult.entities[0].snippet?.fold).toBeDefined();
    expect(foldResult.entities[0].snippet?.preview).toBeUndefined();
    expect(foldResult.entities[0].snippet?.full).toBeUndefined();

    // Test preview mode
    const previewResult = await client.searchEntities({
      query: "sanitize",
      snippetMode: "preview",
    });
    expect(previewResult.entities[0].snippet?.fold).toBeDefined();
    expect(previewResult.entities[0].snippet?.preview).toBeDefined();
    expect(previewResult.entities[0].snippet?.full).toBeUndefined();

    // Test full mode
    const fullResult = await client.searchEntities({
      query: "sanitize",
      snippetMode: "full",
    });
    expect(fullResult.entities[0].snippet?.fold).toBeDefined();
    expect(fullResult.entities[0].snippet?.preview).toBeDefined();
    expect(fullResult.entities[0].snippet?.full).toBeDefined();
  });
});
