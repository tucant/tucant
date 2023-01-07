// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import { search_module } from "../api";
import SearchPage from "../components/SearchPage";

export function SearchModules() {
  return (
    <SearchPage
      title="Modulsuche"
      base_path="module"
      function={search_module}
    />
  );
}
