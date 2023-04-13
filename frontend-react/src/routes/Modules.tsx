// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import { useParams } from "react-router-dom";
import useSWR from "swr";
import { get_modules } from "../api";
import { ModuleList } from "../components/ModuleList";
import { TucanUrlLink } from "../components/TucanUrlLink";
import { Link } from "../Navigation";
import InitialFetch from "./InitialFetch";
import SignOut from "./Logout";

export default function Modules() {
  const { id } = useParams();

  const { data } = useSWR(["module_menu", id ?? null], ([_, id]) =>
    get_modules(id),
  );

  if (data === null) {
    return <SignOut />;
  }

  return (
    <main className="container-fluid">
      <h1 className="text-center">{data?.inner.module_menu.name}</h1>
      <InitialFetch url={"/setup-modules"} />

      {data && (
        <>
          {data.inner.path.map((p) => (
            <nav
              key={p.map((pe) => pe.tucan_id).join()}
              style={{ "--bs-breadcrumb-divider": "'>'" }}
              aria-label="breadcrumb"
            >
              <ol className="breadcrumb">
                {p.map((pe) => (
                  <Link
                    key={pe.tucan_id}
                    className="breadcrumb-item active"
                    to={`/modules/${pe.tucan_id}`}
                  >
                    {pe.name}
                  </Link>
                ))}
              </ol>
            </nav>
          ))}
          <TucanUrlLink data={data} />
          <ModuleList listData={data.inner} />
        </>
      )}
    </main>
  );
}
