<p align="center"><strong>Codex CLI</strong> is a local coding agent for external OpenAI-compatible APIs.
<p align="center"><strong>Current version: 20260721.2.0</strong></p>
</br>
If you want Codex in your code editor (VS Code, Cursor, Windsurf), <a href="https://developers.openai.com/codex/ide">install in your IDE.</a></p>

---

## Quickstart

### Installing and running Codex CLI

Codex CLI can be installed via the following package managers:

```shell
# Install using npm
npm install -g @openai/codex
```

```shell
# Install using Homebrew
brew install --cask codex
```

Then simply run `codex` to get started.

<details>
<summary>You can also go to the <a href="https://github.com/openai/codex/releases/latest">latest GitHub Release</a> and download the appropriate binary for your platform.</summary>

Each GitHub Release contains many executables, but in practice, you likely want one of these:

- macOS
  - Apple Silicon/arm64: `codex-aarch64-apple-darwin.tar.gz`
  - x86_64 (older Mac hardware): `codex-x86_64-apple-darwin.tar.gz`
- Linux
  - x86_64: `codex-x86_64-unknown-linux-musl.tar.gz`
  - arm64: `codex-aarch64-unknown-linux-musl.tar.gz`

Each archive contains a single entry with the platform baked into the name (e.g., `codex-x86_64-unknown-linux-musl`), so you likely want to rename it to `codex` after extracting it.

</details>

### Configure an external API provider

Codex does not provide account login. Configure an OpenAI-compatible external API through environment variables or `~/.codex/config.toml` before starting the CLI. See [External API providers](./docs/external-api-providers.md) for the supported setup.

## Docs

- [**Codex Documentation**](https://developers.openai.com/codex)
- [**Contributing**](./docs/contributing.md)
- [**Installing & building**](./docs/install.md)
- [**Open source fund**](./docs/open-source-fund.md)

## Versioning

Codex releases use `YYYYMMDD.RELEASE.PATCH`. `RELEASE` starts at `1` for the first release of a day and increments for later releases on the same day; `PATCH` starts at `0` and increments for fixes to that release line. The canonical product version is stored in [`VERSION`](./VERSION) and is used directly by Cargo and npm.

For example, `20260721.2.0` is the second release on July 21, 2026, with no patch increment.

This repository is licensed under the [Apache-2.0 License](LICENSE).
