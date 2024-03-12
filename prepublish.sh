#!/usr/bin/env bash

# 在此issue未解决前，手动处理 https://github.com/rustwasm/wasm-pack/issues/1206

set -eo pipefail # https://stackoverflow.com/a/2871034

cd pkg

npm pkg set files[]='snippets/'