import { defineConfig } from "vite";
import solid from "vite-plugin-solid";
import unocss from "unocss/vite";
import eslint from "vite-plugin-eslint";

export default defineConfig({
    plugins: [solid(), unocss(), eslint()],
    build: {
        target: "esnext",
        manifest: true,
    },
});
