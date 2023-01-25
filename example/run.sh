#!/usr/bin/env bash
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
export PATH="$(realpath "$SCRIPT_DIR/../target/debug"):$PATH"
cargo clean
cargo hirtest "$@"
