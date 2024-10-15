import { defineConfig } from "vite";
import wasm from "vite-plugin-wasm";

export default defineConfig({
    plugins: [
        wasm(),
    ],
    build: {
        lib: {
            entry: "./pkg/tucan_injector.js",
            formats: ["es"]
        },
        target: "esnext"
    }
});