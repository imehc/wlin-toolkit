#!/bin/bash
# Install Rust and wasm-pack for Vercel deployment
export HOME=/root
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source /root/.cargo/env
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
export PATH="/root/.cargo/bin:$PATH"
wasm-pack build --target web
astro build