// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import { useState, useTransition } from "react";
import { index } from "../api";
import useSWR from "swr";

export default function Welcome() {
  const [value, setValue] = useState("welcome");

  // TODO FIXME our key needs to be unique across all components?
  const { data, error } = useSWR(value, {
    fetcher: () => index(null),
  });

  const [isPending, startTransition] = useTransition();

  return (
    <div className="App">
      <h1>Welcome</h1>
      <button
        onClick={() => {
          startTransition(() => {
            setValue(Math.random().toString());
          });
        }}
      >
        TEST {isPending ? "LOADING" : ""}
      </button>

      <ul>{JSON.stringify(data)}</ul>
    </div>
  );
}
