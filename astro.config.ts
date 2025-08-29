import { defineConfig } from 'astro/config'
import { fileURLToPath } from 'url'
import { dirname, resolve } from 'path'
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";
import tailwindcss from '@tailwindcss/vite';

const __filename = fileURLToPath(import.meta.url)
const __dirname = dirname(__filename)

export default defineConfig({
  prefetch: true,
  srcDir: './ui',
  publicDir: './ui/public',
  outDir: './dist',
  integrations: [],
  vite: {
    plugins: [wasm(), topLevelAwait(), tailwindcss()],
    server: {
      fs: {
        allow: ['..']
      }
    },
    build: {
      target: 'esnext'
    },
    resolve: {
      extensions: ['.ts', '.js', '.wasm'],
      alias: {
        '~': resolve(__dirname, 'ui'),
        '#': resolve(__dirname, 'pkg')
      }
    },
    assetsInclude: ['**/*.wasm']
  }
})