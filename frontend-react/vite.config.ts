// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import { defineConfig } from "vite";
import react from "@vitejs/plugin-react-swc";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    react(),
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
    proxy: {
      "/redirect": "http://127.0.0.1:8080/",
    },
  },
});
