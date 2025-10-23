import { JSONRPCClient } from "./client/jsonrpc";
import { loadConfig, type CDSAgentConfig } from "./utils/config";

const config = loadConfig();

const LOG_LEVELS: Record<Exclude<CDSAgentConfig["logLevel"], "silent">, number> = {
  debug: 10,
  info: 20,
  warn: 30,
  error: 40,
};

const shouldLog = (level: Exclude<CDSAgentConfig["logLevel"], "silent">): boolean => {
  if (config.logLevel === "silent") {
    return false;
  }
  const threshold = LOG_LEVELS[config.logLevel as Exclude<CDSAgentConfig["logLevel"], "silent">];
  return threshold !== undefined && LOG_LEVELS[level] >= threshold;
};

const jsonRpcClient = new JSONRPCClient(config.serviceUrl, {
  timeoutMs: config.timeoutMs,
  retryDelaysMs: config.retryDelaysMs,
  logger: (event, payload) => {
    const levelMap: Record<string, Exclude<CDSAgentConfig["logLevel"], "silent">> = {
      request: "debug",
      response: "debug",
      networkError: "warn",
      error: "error",
    };

    if ((event === "request" || event === "response") && !config.enableRequestLogging) {
      return;
    }

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
  console.log(`JSON-RPC service: ${config.serviceUrl}`);

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
