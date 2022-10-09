// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import { useState, useEffect } from "react";
import { useLocation } from "react-router-dom";
import useSWR from "swr";
import { Module, my_modules } from "../api";
import { Link } from "../Navigation";

export default function MyModules() {
  const { data, error } = useSWR("my-modules", {
    fetcher: () => my_modules(null),
  });

  return (
    <div className="container">
      <h1 className="text-center">Meine Module</h1>
      <div className="list-group">
        {data != null &&
          data.map((e) => (
            <Link
              key={e.tucan_id}
              className="list-group-item list-group-item-action"
              to={`/module/${e.tucan_id}`}
            >
              {e.title}
            </Link>
          ))}
      </div>
    </div>
  );
}
