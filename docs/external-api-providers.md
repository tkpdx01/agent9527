# External API providers

Codex runs in external API-only mode. Account login and account-specific commands are not part of the supported product surface.

## Environment-based setup

Set an OpenAI-compatible Responses API endpoint and model before launching Codex:

```sh
export CODEX_API_BASE_URL="https://gateway.example/v1"
export CODEX_API_KEY="your-api-key"
export CODEX_MODEL="your-model-id"
codex
```

`OPENAI_BASE_URL` and `OPENAI_API_KEY` are accepted as compatibility aliases. The API key is read from the environment and is never written to `config.toml`. Endpoints that do not require authentication may omit the key variable.

## Config-based setup

For persistent or advanced provider configuration, define a provider in `~/.codex/config.toml`:

```toml
model = "your-model-id"
model_provider = "company-gateway"

[model_providers.company-gateway]
name = "Company Gateway"
base_url = "https://gateway.example/v1"
env_key = "CODEX_API_KEY"
wire_api = "responses"
requires_openai_auth = false
```

Then export the key and start Codex:

```sh
export CODEX_API_KEY="your-api-key"
codex
```

To select an already configured provider without changing `config.toml`, set `CODEX_MODEL_PROVIDER`:

```sh
export CODEX_MODEL_PROVIDER="company-gateway"
export CODEX_MODEL="your-model-id"
codex
```

`CODEX_MODEL_PROVIDER` and `CODEX_API_BASE_URL` are mutually exclusive. Provider definitions used by Codex must set `requires_openai_auth = false`.

## Upstream isolation

The external API-only behavior is implemented as a distribution policy in `codex-rs/features/src/product_policy.rs` and `codex-cli/lib/product-policy.js`. The native CLI activates the Rust policy before loading configuration, while the npm launcher also exports `CODEX_EXTERNAL_API_ONLY=1` for packaged execution. The upstream authentication implementation remains isolated and unreachable from the Codex command and onboarding surfaces, reducing conflicts when incorporating Codex updates.
