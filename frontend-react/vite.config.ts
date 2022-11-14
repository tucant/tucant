// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import { visualizer } from "rollup-plugin-visualizer";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    react({
      //fastRefresh: false,
    }),
    visualizer({}),
  ],
  server: {
    // https://github.com/vitejs/vite/issues/3301
    proxy: {
      "/redirect": "http://127.0.0.1:8080/",
    },
  },
});
/*
✓ 1008 modules transformed.
dist/index.html                 0.76 KiB
dist/assets/index.f34a4907.js   426.20 KiB / gzip: 137.37 KiB
*/
