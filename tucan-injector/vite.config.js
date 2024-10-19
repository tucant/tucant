import { defineConfig } from "vite";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

export default defineConfig({
    plugins: [
        wasm(),
        topLevelAwait()
    ],
    build: {
        target: "esnext",
        lib: {
            name: "tucan_injector",
            entry: "./src/main.js",
            formats: ["es"]
        },
        assetsInlineLimit: (filePath, content) => true,
        minify: false,
    }
});