#!/bin/bash
# Install Rust toolchain + wasm-pack, ensure wasm32 target, then build the site.
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

# Ensure the wasm32-unknown-unknown target is installed.
# Vercel ships a system Rust (not rustup-managed); for that case we install
# the target manually from the official rust-std archive matching the rustc version.
TARGET="wasm32-unknown-unknown"
SYSROOT="$(rustc --print sysroot)"
if [ -d "$SYSROOT/lib/rustlib/$TARGET" ]; then
  echo "wasm target already present: $TARGET"
elif command -v rustup >/dev/null 2>&1; then
  rustup target add "$TARGET"
else
  echo "rustup not found, installing $TARGET manually"
  RUST_VER="$(rustc --version | sed -E 's/rustc ([0-9]+\.[0-9]+\.[0-9]+).*/\1/')"
  TMP="$(mktemp -d)"
  curl --proto '=https' --tlsv1.2 -sSf \
    "https://static.rust-lang.org/dist/rust-std-${RUST_VER}-${TARGET}.tar.gz" \
    -o "$TMP/rust-std.tgz"
  tar xzf "$TMP/rust-std.tgz" -C "$TMP"
  SRC="$TMP/rust-std-${RUST_VER}-${TARGET}/rust-std-${TARGET}/lib/rustlib/${TARGET}"
  cp -r "$SRC" "$SYSROOT/lib/rustlib/$TARGET"
  rm -rf "$TMP"
  echo "installed $TARGET into $SYSROOT/lib/rustlib"
fi

wasm-pack build --target bundler

pnpm exec astro build
