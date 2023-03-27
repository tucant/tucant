// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import useSWR from "swr";
import { my_modules } from "../api";
import { TucanUrlLink } from "../components/TucanUrlLink";
import { Link } from "../Navigation";
import SignOut from "./Logout";

export default function MyModules() {
  const { data } = useSWR("my-modules", () => my_modules(null));

  if (data === null) {
    return <SignOut />;
  }

  return (
    <div className="container">
      <h1 className="text-center">Meine Module</h1>
      <div className="list-group">
        {data != null && (
          <>
            <TucanUrlLink data={data} />
            {data.inner.map((e) => (
              <Link
                key={e.tucan_id}
                className="list-group-item list-group-item-action"
                to={`/module/${e.tucan_id}`}
              >
                {e.title}
              </Link>
            ))}
          </>
        )}
      </div>
    </div>
  );
}
