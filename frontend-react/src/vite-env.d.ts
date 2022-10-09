// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

/// <reference types="vite/client" />

import type * as CSS from "csstype";

declare module "csstype" {
  interface Properties {
    "--bs-breadcrumb-divider"?: string;
  }
}
