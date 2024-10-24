import { defineConfig } from "vite";
import wasm from "vite-plugin-wasm";

export default defineConfig({
    plugins: [
        wasm(),
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
        rollupOptions: {
            output: {
                banner: `// ==UserScript==
// @name         New Userscript
// @namespace    https://www.tucan.tu-darmstadt.de
// @version      2024-10-18
// @description  try to take over the world!
// @author       You
// @match        https://www.tucan.tu-darmstadt.de/*
// @run-at       document-start
// ==/UserScript==`
            },
        }
    }
});