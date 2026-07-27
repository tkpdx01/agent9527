const EXTERNAL_API_ONLY_ENV = "AGENT9527_EXTERNAL_API_ONLY";
const EXTERNAL_PROVIDER_ID = "agent9527-external";

const TOP_LEVEL_COMMANDS = new Set([
  "exec",
  "review",
  "login",
  "logout",
  "mcp",
  "plugin",
  "mcp-server",
  "app-server",
  "remote-control",
  "app",
  "completion",
  "update",
  "doctor",
  "sandbox",
  "debug",
  "execpolicy",
  "apply",
  "resume",
  "archive",
  "delete",
  "unarchive",
  "fork",
  "cloud",
  "responses-api-proxy",
  "stdio-to-uds",
  "exec-server",
  "features",
]);

const ACCOUNT_COMMANDS = new Set(["login", "logout"]);

export function prepareThirdPartyApiLaunch(args, sourceEnv = process.env) {
  const env = {
    ...sourceEnv,
    [EXTERNAL_API_ONLY_ENV]: "1",
  };
  const command = findTopLevelCommand(args);
  if (ACCOUNT_COMMANDS.has(command)) {
    throw new Error(
      "Agent9527 does not support account login. Configure an external API provider instead.",
    );
  }

  const providerId = sourceEnv.AGENT9527_MODEL_PROVIDER?.trim();
  const baseUrl = (
    sourceEnv.AGENT9527_API_BASE_URL || sourceEnv.OPENAI_BASE_URL || ""
  ).trim();
  if (providerId && baseUrl) {
    throw new Error(
      "Set either AGENT9527_MODEL_PROVIDER or AGENT9527_API_BASE_URL, not both.",
    );
  }
  if (providerId?.toLowerCase() === "openai") {
    throw new Error(
      "The built-in OpenAI account provider is unavailable. Select an external provider with requires_openai_auth = false.",
    );
  }

  const overrides = [];
  if (providerId) {
    overrides.push(configOverride("model_provider", providerId));
  } else if (baseUrl) {
    validateBaseUrl(baseUrl);
    overrides.push(
      configOverride("model_provider", EXTERNAL_PROVIDER_ID),
      configOverride(
        `model_providers.${EXTERNAL_PROVIDER_ID}.name`,
        "External API",
      ),
      configOverride(
        `model_providers.${EXTERNAL_PROVIDER_ID}.base_url`,
        baseUrl,
      ),
      [
        "-c",
        `model_providers.${EXTERNAL_PROVIDER_ID}.requires_openai_auth=false`,
      ],
    );

    const apiKeyEnv = sourceEnv.AGENT9527_API_KEY
      ? "AGENT9527_API_KEY"
      : sourceEnv.OPENAI_API_KEY
        ? "OPENAI_API_KEY"
        : null;
    if (apiKeyEnv) {
      overrides.push(
        configOverride(
          `model_providers.${EXTERNAL_PROVIDER_ID}.env_key`,
          apiKeyEnv,
        ),
      );
    }
  }

  const model = sourceEnv.AGENT9527_MODEL?.trim();
  if (model) {
    overrides.push(configOverride("model", model));
  }

  return {
    args: [...overrides.flat(), ...args],
    env,
  };
}

function findTopLevelCommand(args) {
  for (const arg of args) {
    if (arg === "--") {
      return null;
    }
    if (TOP_LEVEL_COMMANDS.has(arg)) {
      return arg;
    }
  }
  return null;
}

function configOverride(key, value) {
  return ["-c", `${key}=${JSON.stringify(value)}`];
}

function validateBaseUrl(value) {
  let parsed;
  try {
    parsed = new URL(value);
  } catch {
    throw new Error(`Invalid external API base URL: ${value}`);
  }
  if (parsed.protocol !== "http:" && parsed.protocol !== "https:") {
    throw new Error(
      `External API base URL must use http or https: ${value}`,
    );
  }
}

export const productPolicy = Object.freeze({
  externalApiOnlyEnv: EXTERNAL_API_ONLY_ENV,
  externalProviderId: EXTERNAL_PROVIDER_ID,
});
