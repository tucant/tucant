// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import { Breadcrumbs, Link } from "@mui/material";
import Alert from "@mui/material/Alert";
import LinearProgress from "@mui/material/LinearProgress";
import List from "@mui/material/List";
import Typography from "@mui/material/Typography";
import { useState, useEffect } from "react";
import { useLocation } from "react-router-dom";
import { RouterLink } from "../MiniDrawer";
import InitialFetch from "./InitialFetch";
import NavigateNextIcon from "@mui/icons-material/NavigateNext";
import { ModuleMenuResponse } from "../validation-io-ts";
import { isLeft } from "fp-ts/lib/Either";
import { PathReporter } from "io-ts/lib/PathReporter";
import { TypeOf } from "io-ts";

export default function Modules() {
  const location = useLocation();

  const [data, setData] = useState<TypeOf<typeof ModuleMenuResponse> | null>(
    null
  );
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const getData = async () => {
      const response = await fetch(
        `http://localhost:8080${location.pathname}`,
        {
          credentials: "include",
        }
      );
      if (!response.ok) {
        throw new Error(
          `This is an HTTP error: The status is ${
            response.status
          }. ${await response.text()}`
        );
      }
      const actualData = ModuleMenuResponse.decode(await response.json());
      if (isLeft(actualData)) {
        throw new Error(
          `Internal Error: Invalid data format in response ${PathReporter.report(
            actualData
          ).join("\n")}`
        );
      }
      setData(actualData.right);
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
      <Typography variant="h2">Module</Typography>
      {loading && <LinearProgress />}
      {error && <Alert severity="error">{error}</Alert>}
      {data?.path.map((p, i) => (
        <Breadcrumbs
          key={i}
          separator={<NavigateNextIcon fontSize="small" />}
          aria-label="breadcrumb"
        >
          {p.map((pe) => (
            <Link
              underline="hover"
              key={pe.tucan_id}
              color="inherit"
              href={`/modules/${pe.tucan_id}`}
            >
              {pe.name}
            </Link>
          ))}
        </Breadcrumbs>
      ))}
      <List>
        {data != null &&
          "Submenu" in data.entries &&
          data.entries.Submenu.map((e) => (
            <RouterLink
              key={e.tucan_id}
              to={`/modules/${e.tucan_id}`}
              text={e.name}
            ></RouterLink>
          ))}
        {data != null &&
          "Modules" in data.entries &&
          data.entries.Modules.map((e) => (
            <RouterLink
              key={e.tucan_id}
              to={`/module/${e.tucan_id}`}
              text={e.title}
            ></RouterLink>
          ))}
      </List>

      <InitialFetch></InitialFetch>
    </>
  );
}
