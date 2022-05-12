/// <reference types="vitest" />
/// <reference types="vitest/globals" />

import { defineConfig } from "vite";
import solidPlugin from "vite-plugin-solid";

export default defineConfig({
  test: {
    globals: true,
    environment: "jsdom",
    reporters: "dot",
    deps: {
      inline: [/solid-js/],
    },
    transformMode: {
      web: [/\.[jt]sx?$/],
    },
  },
  resolve: {
    conditions: ['development', 'browser'],
  },
  plugins: [solidPlugin()],
});
