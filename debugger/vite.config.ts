import vue from "@vitejs/plugin-vue";
import { defineConfig } from "vite";
import ViteFonts from "vite-plugin-fonts";
import WindiCSS from "vite-plugin-windicss";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    ViteFonts({
      google: {
        families: ["Roboto:400,500,700"],
      },
    }),
    WindiCSS(),
    vue(),
  ],
});
