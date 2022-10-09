// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import { search_module } from "../api";
import SearchPage from "../components/SearchPage";

export function SearchModules() {
  return (
    <SearchPage
      title="Modulsuche"
      key="search-modules"
      function={search_module}
    ></SearchPage>
  );
}
