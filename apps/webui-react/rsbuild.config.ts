import { defineConfig } from '@rsbuild/core';
import { pluginReact } from '@rsbuild/plugin-react';
import { pluginSass } from '@rsbuild/plugin-sass';
import { version } from "./package.json";

export default defineConfig({
  plugins: [
    pluginReact(),
    pluginSass(),
  ],
  html: {
    template: './index.html',
  },
  source: {
    entry: {
      index: './src/main.tsx',
    },
    decorators: {
      version: "2022-03"
    },
    define: {
      'VERSION': JSON.stringify(version),
      'process.env.NODE_ENV': JSON.stringify(process.env.NODE_ENV),
    }
  },
  // Rsbuild options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent rsbuild from obscuring rust errors
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true
  },
  // for advanced configuration
  tools: {
    rspack: {
    }
  }
})