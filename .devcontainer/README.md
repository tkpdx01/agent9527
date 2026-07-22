# Containerized Development

We provide two container paths:

- `devcontainer.json` keeps the existing Agent9527 contributor setup for working on this repository.
- `devcontainer.secure.json` adds a customer-oriented profile with stricter outbound network controls.

## Agent9527 contributor profile

Use `devcontainer.json` when you are developing Agent9527 itself. This is the same lightweight arm64 container that already exists in the repo.

## Secure customer profile

Use `devcontainer.secure.json` when you want a stricter runtime profile for running Agent9527 inside a project container:

- installs the Agent9527 CLI plus common build tools
- installs bubblewrap in setuid mode for Agent9527's Linux sandbox
- disables Docker's outer seccomp and AppArmor profiles so bubblewrap can construct Agent9527's inner sandbox
- enables firewall startup with an allowlist-driven outbound policy
- blocks IPv6 by default so the allowlist cannot be bypassed over AAAA routes
- requires `NET_ADMIN` and `NET_RAW` so the firewall can be installed at startup

This profile keeps the stricter networking isolated to the customer path instead of changing the default Agent9527 contributor container.

Start it from the CLI with:

```bash
devcontainer up --workspace-folder . --config .devcontainer/devcontainer.secure.json
```

In VS Code, choose **Dev Containers: Open Folder in Container...** and select `.devcontainer/devcontainer.secure.json`.

## Docker

To build the contributor image locally for x64 and then run it with the repo mounted under `/workspace`:

```shell
AGENT9527_DOCKER_IMAGE_NAME=agent9527-linux-dev
docker build --platform=linux/amd64 -t "$AGENT9527_DOCKER_IMAGE_NAME" ./.devcontainer
docker run --platform=linux/amd64 --rm -it -e CARGO_TARGET_DIR=/workspace/agent9527-rs/target-amd64 -v "$PWD":/workspace -w /workspace/agent9527-rs "$AGENT9527_DOCKER_IMAGE_NAME"
```

Note that `/workspace/target` will contain the binaries built for your host platform, so we include `-e CARGO_TARGET_DIR=/workspace/agent9527-rs/target-amd64` in the `docker run` command so that the binaries built inside your container are written to a separate directory.

For arm64, specify `--platform=linux/arm64` instead for both `docker build` and `docker run`.

Currently, the contributor `Dockerfile` works for both x64 and arm64 Linux, though you need to run `rustup target add x86_64-unknown-linux-musl` yourself to install the musl toolchain for x64.

The secure profile's capability, seccomp, and AppArmor options are required when you want Agent9527's bubblewrap sandbox to run inside Docker as the non-root devcontainer user. Without them, Docker's default runtime profile can block bubblewrap's namespace setup before Agent9527's own seccomp filter is installed. This keeps the Docker relaxation explicit in the profile that is meant to run Agent9527 inside a project container, while the default contributor profile stays lightweight.
