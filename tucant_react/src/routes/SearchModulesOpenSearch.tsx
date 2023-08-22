// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import { search_module_opensearch } from "../api";
import SearchPage from "../components/SearchPage";

export function SearchModulesOpenSearch() {
  return (
    <SearchPage
      title="Modulsuche (OpenSearch)"
      base_path="module"
      function={search_module_opensearch}
    />
  );
}
