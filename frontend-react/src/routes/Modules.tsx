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

  const { data, error } = useSWR(["module_menu", id ?? null], {
    fetcher: (_, id) => get_modules(id),
  });

  return (
    <>
      <h1>{data?.module_menu.name}</h1>
      {data?.path.map((p, i) => (
        <div key={i} aria-label="breadcrumb">
          {p.map((pe) => (
            <Link
              key={pe.tucan_id}
              color="inherit"
              to={`/modules/${pe.tucan_id}`}
            >
              {pe.name}
            </Link>
          ))}
        </div>
      ))}
      {data && <ModuleList listData={data} />}
    </>
  );
}
