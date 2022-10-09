// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import { useParams } from "react-router-dom";
import useSWR from "swr";
import { get_modules } from "../api";
import { ModuleList } from "../components/ModuleList";
import { Link } from "../Navigation";

export default function Modules() {
  const { id } = useParams();

  const { data } = useSWR(["module_menu", id ?? null], {
    fetcher: (_, id) => get_modules(id),
  });

  return (
    <main className="container">
      <h1 className="text-center">{data?.module_menu.name}</h1>
      {data?.path.map((p, i) => (
        <nav
          key={i}
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
      {data && <ModuleList listData={data} />}
    </main>
  );
}
