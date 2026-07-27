import assert from "node:assert/strict";
import test from "node:test";

import {
  prepareThirdPartyApiLaunch,
  productPolicy,
} from "../lib/product-policy.js";

test("enables external API-only enforcement", () => {
  const launch = prepareThirdPartyApiLaunch(["--help"], {});

  assert.deepEqual(launch.args, ["--help"]);
  assert.equal(launch.env[productPolicy.externalApiOnlyEnv], "1");
});

test("rejects top-level account commands", () => {
  for (const command of ["login", "logout"]) {
    assert.throws(
      () => prepareThirdPartyApiLaunch([command], {}),
      /does not support account login/,
    );
  }
});

test("does not reject MCP OAuth commands", () => {
  const launch = prepareThirdPartyApiLaunch(["mcp", "login", "example"], {});

  assert.deepEqual(launch.args, ["mcp", "login", "example"]);
});

test("injects an external provider from environment variables", () => {
  const launch = prepareThirdPartyApiLaunch(["exec", "fix tests"], {
    CODEX_API_BASE_URL: "https://gateway.example/v1",
    CODEX_API_KEY: "secret",
    CODEX_MODEL: "model-x",
  });

  assert.deepEqual(launch.args, [
    "-c",
    'model_provider="codex-external"',
    "-c",
    'model_providers.codex-external.name="External API"',
    "-c",
    'model_providers.codex-external.base_url="https://gateway.example/v1"',
    "-c",
    "model_providers.codex-external.requires_openai_auth=false",
    "-c",
    'model_providers.codex-external.env_key="CODEX_API_KEY"',
    "-c",
    'model="model-x"',
    "exec",
    "fix tests",
  ]);
});

test("supports selecting a provider defined in config.toml", () => {
  const launch = prepareThirdPartyApiLaunch(["review"], {
    CODEX_MODEL_PROVIDER: "company-gateway",
    CODEX_MODEL: "review-model",
  });

  assert.deepEqual(launch.args, [
    "-c",
    'model_provider="company-gateway"',
    "-c",
    'model="review-model"',
    "review",
  ]);
});

test("allows auth-free local endpoints", () => {
  const launch = prepareThirdPartyApiLaunch([], {
    OPENAI_BASE_URL: "http://127.0.0.1:8080/v1",
  });

  assert.equal(
    launch.args.includes(
      'model_providers.codex-external.env_key="CODEX_API_KEY"',
    ),
    false,
  );
});

test("rejects ambiguous provider configuration", () => {
  assert.throws(
    () =>
      prepareThirdPartyApiLaunch([], {
        CODEX_MODEL_PROVIDER: "company-gateway",
        CODEX_API_BASE_URL: "https://gateway.example/v1",
      }),
    /either CODEX_MODEL_PROVIDER or CODEX_API_BASE_URL/,
  );
});

test("rejects the built-in account provider", () => {
  assert.throws(
    () =>
      prepareThirdPartyApiLaunch([], {
        CODEX_MODEL_PROVIDER: "openai",
      }),
    /built-in OpenAI account provider is unavailable/,
  );
});
