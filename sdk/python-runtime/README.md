# Agent9527 CLI Runtime for Python SDK

Platform-specific runtime package consumed by the published `openai-agent9527`.

This package is staged during release so the SDK can pin an exact Agent9527 CLI
version without checking platform binaries into the repo.

`openai-agent9527-cli-bin` is intentionally wheel-only. Do not build or publish an
sdist for this package.
