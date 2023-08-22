// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import { search_course } from "../api";
import SearchPage from "../components/SearchPage";

export function SearchCourses() {
  return (
    <SearchPage
      title="Veranstaltungssuche"
      base_path="course"
      function={search_course}
    />
  );
}
