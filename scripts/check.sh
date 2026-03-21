#!/usr/bin/env bash

set -euo pipefail

cargo fmt --check
cargo test
cargo check --example digitwise_number_editor
