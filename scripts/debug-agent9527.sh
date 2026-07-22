#!/bin/bash

# Set "chatgpt.cliExecutable": "/Users/<USERNAME>/code/agent9527/scripts/debug-agent9527.sh" in VSCode settings to always get the
# latest agent9527-rs binary when debugging Agent9527 Extension.


set -euo pipefail

AGENT9527_RS_DIR=$(realpath "$(dirname "$0")/../agent9527-rs")
(cd "$AGENT9527_RS_DIR" && cargo run --quiet --bin agent9527 -- "$@")