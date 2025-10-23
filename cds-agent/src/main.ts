import { JSONRPCClient } from "./client/jsonrpc";

type LogLevel = "debug" | "info" | "warn" | "error" | "silent";

const LOG_LEVELS: Record<Exclude<LogLevel, "silent">, number> = {
  debug: 10,
  info: 20,
  warn: 30,
  error: 40,
};

const resolveLogLevel = (value: string | undefined): LogLevel => {
  if (!value) {
    return "info";
  }
  const normalized = value.toLowerCase();
  if (["debug", "info", "warn", "error", "silent"].includes(normalized)) {
    return normalized as LogLevel;
  }
  return "info";
};

const currentLogLevel = resolveLogLevel(process.env.LOG_LEVEL);

const shouldLog = (level: Exclude<LogLevel, "silent">): boolean => {
  if (currentLogLevel === "silent") {
    return false;
  }
  const threshold = LOG_LEVELS[currentLogLevel as Exclude<LogLevel, "silent">];
  return threshold !== undefined && LOG_LEVELS[level] >= threshold;
};

const serviceUrl = process.env.CDS_INDEX_SERVICE_URL ?? "http://localhost:9876/rpc";

const jsonRpcClient = new JSONRPCClient(serviceUrl, {
  logger: (event, payload) => {
    const levelMap: Record<string, Exclude<LogLevel, "silent">> = {
      request: "debug",
      response: "debug",
      networkError: "warn",
      error: "error",
    };
    const level = levelMap[event] ?? "debug";
    if (!shouldLog(level)) {
      return;
    }
    const prefix = `[jsonrpc:${event}]`;
    if (level === "error") {
      console.error(prefix, payload);
    } else if (level === "warn") {
      console.warn(prefix, payload);
    } else if (level === "info") {
      console.info(prefix, payload);
    } else {
      console.debug(prefix, payload);
    }
  },
});

async function bootstrap(): Promise<void> {
  console.log("CDSAgent v0.1.0 - LLM Orchestration Layer");
  console.log(`JSON-RPC service: ${serviceUrl}`);

  // TODO: Initialize Claude Agent SDK and register hooks once available.
  // The JSONRPCClient instance is ready to be injected into agent tool handlers.

  if (shouldLog("debug")) {
    console.debug("JSON-RPC client configured with retry strategy.");
  }
}

void bootstrap().catch((error) => {
  console.error("Failed to bootstrap CDSAgent", error);
  process.exitCode = 1;
});

export { jsonRpcClient };
