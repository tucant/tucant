// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import { useState, useEffect } from "react";
import { index } from "../api";
import useSWR, { useSWRConfig } from "swr";

export default function Welcome() {
  // TODO FIXME our key needs to be unique across all components?
  const { data, error } = useSWR("welcome", {
    fetcher: () => index(null),
  });

  return (
    <div className="App">
      <h1>Welcome</h1>
      <ul>{JSON.stringify(data)}</ul>
    </div>
  );
}
