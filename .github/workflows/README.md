# Workflow Strategy

The workflows in this directory are split so that pull requests get fast, review-friendly signal while `main` still gets the full cross-platform verification pass.

## Agent9527 Automation

`upstream-sync.yml` checks OpenAI Codex `main` every day, performs a history-preserving merge,
and uses Codex CLI to resolve conflicts and adapt new upstream code to Agent9527 branding. Configure
these repository settings before enabling it:

- Actions secrets `AGENT9527_SYNC_API_KEY` and `AGENT9527_SYNC_BASE_URL`.
- Optional Actions variable `AGENT9527_SYNC_MODEL`; when omitted, the workflow selects a suitable
  GPT-5/Codex model from the provider's `/models` response.

Successful syncs bump the calendar version, update `.github/upstream.json`, push `main`, tag the
release, and dispatch `agent9527-publish.yml`.

`agent9527-publish.yml` builds native npm payloads for Linux, macOS, and Windows on x64 and arm64,
then publishes `@tkpdx01/agent9527`. npm Trusted Publishing is preferred; configure the trusted
publisher for repository `tkpdx01/agent9527`, workflow `agent9527-publish.yml`, and environment
`npm`. Alternatively, add an `NPM_TOKEN` Actions secret.

## Pull Requests

- `bazel.yml` is the main pre-merge verification path for Rust code.
  It runs Bazel `test` and Bazel `clippy` on the supported Bazel targets,
  including the generated Rust test binaries needed to lint inline `#[cfg(test)]`
  code.
- `rust-ci.yml` keeps the Cargo-native PR checks intentionally small:
  - `cargo fmt --check`
  - `cargo shear`
  - `argument-comment-lint` on Linux, macOS, and Windows
  - `tools/argument-comment-lint` package tests when the lint or its workflow wiring changes

## Post-Merge On `main`

- `bazel.yml` also runs on pushes to `main`.
  This re-verifies the merged Bazel path and helps keep the BuildBuddy caches warm.
- `rust-ci-full.yml` is the full Cargo-native verification workflow.
  It keeps the heavier checks off the PR path while still validating them after merge:
  - the full Cargo `clippy` matrix
  - the full Cargo `nextest` matrix via per-platform archive-backed shards
  - Windows ARM64 nextest archives cross-compiled on Windows x64, then replayed on native Windows ARM64 shards
  - release-profile Cargo builds
  - cross-platform `argument-comment-lint`
  - Linux remote-env tests

## Rule Of Thumb

- If a build/test/clippy check can be expressed in Bazel, prefer putting the PR-time version in `bazel.yml`.
- Keep `rust-ci.yml` fast enough that it usually does not dominate PR latency.
- Reserve `rust-ci-full.yml` for heavyweight Cargo-native coverage that Bazel does not replace yet.
