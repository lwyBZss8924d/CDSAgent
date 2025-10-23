/**
 * Configuration management for CDSAgent.
 * Centralized configuration loading from environment variables with defaults.
 */

export interface CDSAgentConfig {
  /**
   * JSON-RPC service endpoint URL.
   * Default: http://localhost:9876/rpc
   */
  serviceUrl: string;

  /**
   * Request timeout in milliseconds.
   * Default: 30000 (30 seconds)
   */
  timeoutMs: number;

  /**
   * Retry delays in milliseconds for network failures.
   * Default: [1000, 2000, 4000] (exponential backoff)
   */
  retryDelaysMs: number[];

  /**
   * Log level for debugging.
   * Options: debug, info, warn, error, silent
   * Default: info
   */
  logLevel: "debug" | "info" | "warn" | "error" | "silent";

  /**
   * Enable request/response logging.
   * Default: false
   */
  enableRequestLogging: boolean;
}

/**
 * Parse retry delays from comma-separated string.
 */
function parseRetryDelays(value: string | undefined): number[] {
  if (!value) {
    return [1000, 2000, 4000];
  }

  try {
    const delays = value.split(",").map((s) => parseInt(s.trim(), 10));
    if (delays.some((d) => isNaN(d) || d < 0)) {
      console.warn("Invalid retry delays configuration, using defaults");
      return [1000, 2000, 4000];
    }
    return delays;
  } catch {
    console.warn("Failed to parse retry delays, using defaults");
    return [1000, 2000, 4000];
  }
}

/**
 * Parse log level from environment variable.
 */
function parseLogLevel(value: string | undefined): CDSAgentConfig["logLevel"] {
  const validLevels = ["debug", "info", "warn", "error", "silent"];
  const normalized = value?.toLowerCase();

  if (normalized && validLevels.includes(normalized)) {
    return normalized as CDSAgentConfig["logLevel"];
  }

  return "info";
}

/**
 * Load configuration from environment variables.
 *
 * Environment variables:
 * - CDS_INDEX_SERVICE_URL: Service endpoint URL
 * - CDS_REQUEST_TIMEOUT_MS: Request timeout in milliseconds
 * - CDS_RETRY_DELAYS_MS: Comma-separated retry delays (e.g., "1000,2000,4000")
 * - CDS_LOG_LEVEL: Log level (debug|info|warn|error|silent)
 * - CDS_ENABLE_REQUEST_LOGGING: Enable request/response logging (true|false)
 */
export function loadConfig(): CDSAgentConfig {
  return {
    serviceUrl: process.env.CDS_INDEX_SERVICE_URL ?? "http://localhost:9876/rpc",
    timeoutMs: parseInt(process.env.CDS_REQUEST_TIMEOUT_MS ?? "30000", 10) || 30000,
    retryDelaysMs: parseRetryDelays(process.env.CDS_RETRY_DELAYS_MS),
    logLevel: parseLogLevel(process.env.CDS_LOG_LEVEL ?? process.env.LOG_LEVEL),
    enableRequestLogging: process.env.CDS_ENABLE_REQUEST_LOGGING === "true",
  };
}

/**
 * Default configuration instance.
 * Can be overridden by calling loadConfig() with different environment variables.
 */
export const defaultConfig = loadConfig();