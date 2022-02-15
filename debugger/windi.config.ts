import colors from "windicss/colors";
import { defineConfig } from "windicss/helpers";
export default defineConfig({
  darkMode: "class",
  theme: {
    extend: {
      colors: {
        primary: colors.violet,
        secondary: colors.sky,
      },
    },
  },
});
