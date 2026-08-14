#!/bin/bash
# Install Rust and wasm-pack for Vercel/build, then build the site.
set -euo pipefail

CARGO_HOME="${CARGO_HOME:-$HOME/.cargo}"
export CARGO_HOME
export PATH="$CARGO_HOME/bin:$PATH"

# Install Rust toolchain if missing
if ! command -v cargo >/dev/null 2>&1; then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
fi

# Install wasm-pack if missing
if ! command -v wasm-pack >/dev/null 2>&1; then
  curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

wasm-pack build --target bundler

pnpm exec astro build
