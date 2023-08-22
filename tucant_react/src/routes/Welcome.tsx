// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import { index } from "../api";
import SignOut from "./Logout";
import useSWR from "swr";

export default function Welcome() {
  // TODO FIXME add user id to cache key
  const { data } = useSWR("welcome", () => index(null));

  if (data === null) {
    return <SignOut />;
  }

  return (
    <main className="container">
      <h1 className="text-center">Welcome</h1>
      <ul>{JSON.stringify(data)}</ul>
    </main>
  );
}
