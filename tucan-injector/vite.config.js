import { defineConfig } from "vite";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

export default defineConfig({
    plugins: [
        wasm(),
        topLevelAwait()
    ],
    build: {
        lib: {
            name: "tucan_injector",
            entry: "./dist/tucan_injector.js",
            formats: ["es"]
        },
        minify: false
    }
});