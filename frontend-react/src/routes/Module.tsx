// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import { useState, useEffect } from "react";
import { useLocation, useParams } from "react-router-dom";
import dompurify from "dompurify";
import { module, ModuleResponse } from "../api";
import useSWR from "swr";
import { Link } from "../Navigation";

export default function Module() {
  const location = useLocation();
  const { id } = useParams();

  const { data, error } = useSWR(["course", id ?? ""], {
    fetcher: (_, id) => module(id),
  });

  return (
    <>
      {data && (
        <>
          <h1>
            {data.module.module_id} {data.module.title}
          </h1>
          {data.path.map((p, i) => (
            <nav key={i} aria-label="breadcrumb">
              <ol className="breadcrumb">
                {p.map((pe) => (
                  <li key={pe.tucan_id} className="breadcrumb-item">
                    <Link to={`/modules/${pe.tucan_id}`}>{pe.name}</Link>
                  </li>
                ))}
              </ol>
            </nav>
          ))}
          <span className="badge rounded-pill text-bg-primary">{`${
            data.module.credits ?? 0
          } Credits`}</span>
          <div
            dangerouslySetInnerHTML={{
              __html: dompurify.sanitize(data.module.content),
            }}
          ></div>
        </>
      )}
    </>
  );
}
