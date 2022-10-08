// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import { useState, useEffect } from "react";
import { Link, useLocation, useParams } from "react-router-dom";
import { get_modules, ModuleMenuResponse } from "../api";
import { ModuleList } from "../components/ModuleList";

export default function Modules() {
  const location = useLocation();

  const [data, setData] = useState<ModuleMenuResponse | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const { id } = useParams();

  useEffect(() => {
    const getData = async () => {
      setData(await get_modules(id || null));
      setError(null);
    };
    getData()
      .catch((err) => {
        setError(String(err));
        setData(null);
      })
      .finally(() => {
        setLoading(false);
      });
  }, [location]);

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
