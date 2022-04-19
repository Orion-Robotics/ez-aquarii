import vue from "@vitejs/plugin-vue";
import IconsResolver from "unplugin-icons/resolver";
import Icons from "unplugin-icons/vite";
import Components from "unplugin-vue-components/vite";
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
    vue(),
    WindiCSS(),
    Components({
      resolvers: [IconsResolver()],
    }),
    Icons({
      compiler: "vue3",
    }),
  ],
});
