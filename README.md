<p align="center"><strong>Agent9527 CLI</strong> is a local coding agent for external OpenAI-compatible APIs.
<p align="center"><strong>Current version: 20260727.1.0</strong></p>
</br>
If you want Agent9527 in your code editor (VS Code, Cursor, Windsurf), <a href="https://developers.openai.com/agent9527/ide">install in your IDE.</a></p>

---

## Quickstart

### Installing and running Agent9527 CLI

Agent9527 CLI can be installed via the following package managers:

```shell
# Install using npm
npm install -g @tkpdx01/agent9527
```

```shell
# Install using Homebrew
brew install --cask agent9527
```

Then simply run `agent9527` to get started.

<details>
<summary>You can also go to the <a href="https://github.com/tkpdx01/agent9527/releases/latest">latest GitHub Release</a> and download the appropriate binary for your platform.</summary>

Each GitHub Release contains many executables, but in practice, you likely want one of these:

- macOS
  - Apple Silicon/arm64: `agent9527-aarch64-apple-darwin.tar.gz`
  - x86_64 (older Mac hardware): `agent9527-x86_64-apple-darwin.tar.gz`
- Linux
  - x86_64: `agent9527-x86_64-unknown-linux-musl.tar.gz`
  - arm64: `agent9527-aarch64-unknown-linux-musl.tar.gz`

Each archive contains a single entry with the platform baked into the name (e.g., `agent9527-x86_64-unknown-linux-musl`), so you likely want to rename it to `agent9527` after extracting it.

</details>

### Configure an external API provider

Agent9527 does not provide account login. Configure an OpenAI-compatible external API through environment variables or `~/.agent9527/config.toml` before starting the CLI. See [External API providers](./docs/external-api-providers.md) for the supported setup.

## Docs

- [**Agent9527 Documentation**](https://developers.openai.com/agent9527)
- [**Contributing**](./docs/contributing.md)
- [**Installing & building**](./docs/install.md)
- [**Open source fund**](./docs/open-source-fund.md)

## Versioning

Agent9527 releases use `YYYYMMDD.RELEASE.PATCH`. `RELEASE` starts at `1` for the first release of a day and increments for later releases on the same day; `PATCH` starts at `0` and increments for fixes to that release line. The canonical product version is stored in [`VERSION`](./VERSION) and is used directly by Cargo and npm.

For example, `20260727.1.0` is the first release on July 27, 2026, with no patch increment.

This repository is licensed under the [Apache-2.0 License](LICENSE).
