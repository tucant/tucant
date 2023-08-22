// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import { my_modules } from "../api";
import { ModuleEntry } from "../components/ModuleEntry";
import { TucanUrlLink } from "../components/TucanUrlLink";
import SignOut from "./Logout";
import useSWR from "swr";

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
              <ModuleEntry key={e.value.tucan_id} module={e} />
            ))}
          </>
        )}
      </div>
    </div>
  );
}
