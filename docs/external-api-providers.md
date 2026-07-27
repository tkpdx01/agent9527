# External API providers

Agent9527 runs in external API-only mode. Account login and account-specific commands are not part of the supported product surface.

## Environment-based setup

Set an OpenAI-compatible Responses API endpoint and model before launching Agent9527:

```sh
export AGENT9527_API_BASE_URL="https://gateway.example/v1"
export AGENT9527_API_KEY="your-api-key"
export AGENT9527_MODEL="your-model-id"
agent9527
```

`OPENAI_BASE_URL` and `OPENAI_API_KEY` are accepted as compatibility aliases. The API key is read from the environment and is never written to `config.toml`. Endpoints that do not require authentication may omit the key variable.

## Config-based setup

For persistent or advanced provider configuration, define a provider in `~/.agent9527/config.toml`:

```toml
model = "your-model-id"
model_provider = "company-gateway"

[model_providers.company-gateway]
name = "Company Gateway"
base_url = "https://gateway.example/v1"
env_key = "AGENT9527_API_KEY"
wire_api = "responses"
requires_openai_auth = false
```

Then export the key and start Agent9527:

```sh
export AGENT9527_API_KEY="your-api-key"
agent9527
```

To select an already configured provider without changing `config.toml`, set `AGENT9527_MODEL_PROVIDER`:

```sh
export AGENT9527_MODEL_PROVIDER="company-gateway"
export AGENT9527_MODEL="your-model-id"
agent9527
```

`AGENT9527_MODEL_PROVIDER` and `AGENT9527_API_BASE_URL` are mutually exclusive. Provider definitions used by Agent9527 must set `requires_openai_auth = false`.

## Upstream isolation

The external API-only behavior is implemented as a distribution policy in `agent9527-rs/features/src/product_policy.rs` and `agent9527-cli/lib/product-policy.js`. The native CLI activates the Rust policy before loading configuration, while the npm launcher also exports `AGENT9527_EXTERNAL_API_ONLY=1` for packaged execution. The upstream authentication implementation remains isolated and unreachable from the Agent9527 command and onboarding surfaces, reducing conflicts when incorporating Codex updates.
