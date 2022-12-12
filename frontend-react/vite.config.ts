// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import { defineConfig } from "vite";
import { visualizer } from "rollup-plugin-visualizer";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    visualizer({
      brotliSize: true,
      gzipSize: true,
      template: "sunburst",
      open: true,
    }),
  ],
  build: {
    sourcemap: true,
  },
  server: {
    // https://github.com/vitejs/vite/issues/3301
    proxy: {
      "/redirect": "http://127.0.0.1:8080/",
    },
  },
});
