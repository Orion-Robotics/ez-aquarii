import transformerDirective from "@unocss/transformer-directives";
import UnocssPlugin from "@unocss/vite";
import Icons from "unplugin-icons/vite";
import { defineConfig } from "vite";
import inlineCssModules from "vite-plugin-inline-css-modules";
import solidPlugin from "vite-plugin-solid";

export default defineConfig({
  plugins: [
    solidPlugin({}),
    UnocssPlugin({
      transformers: [transformerDirective()],
    }),
    Icons({
      compiler: "solid",
    }),
    inlineCssModules(),
  ],
  build: {
    target: "esnext",
    polyfillDynamicImport: false,
  },
});
