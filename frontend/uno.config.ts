import { defineConfig, presetWind, presetIcons } from "unocss";

export default defineConfig({
    presets: [
        presetIcons({
            collections: {
                // Load icons on-demand
                fa: () => import("@iconify-json/fa6-solid/icons.json"),
                spinners: () => import("@iconify-json/svg-spinners/icons.json"),
                cib: () => import("@iconify-json/cib/icons.json"),
            },
        }),
        presetWind(),
    ],
    theme: {
        colors: {
            dplime: {
                primary: "#c5ff00",
                dark: "#9ccc00",
            },
        },
    },
});
