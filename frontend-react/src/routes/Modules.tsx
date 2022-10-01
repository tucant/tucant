// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import { Breadcrumbs, Link } from "@mui/material";
import Alert from "@mui/material/Alert";
import LinearProgress from "@mui/material/LinearProgress";
import List from "@mui/material/List";
import Typography from "@mui/material/Typography";
import { useState, useEffect } from "react";
import { useLocation, useParams } from "react-router-dom";
import { RouterLink } from "../MiniDrawer";
import InitialFetch from "./InitialFetch";
import NavigateNextIcon from "@mui/icons-material/NavigateNext";
import { get_modules, ModuleMenuResponse } from "../api";

export default function Modules() {
  const location = useLocation();

  const [data, setData] = useState<ModuleMenuResponse | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const { id } = useParams();

  useEffect(() => {
    const getData = async () => {
      if (!id) {
        throw new Error("Menünummer fehlt!");
      }
      setData(await get_modules(id));
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
      <Typography variant="h4" component="h1">
        {data?.module_menu.name}
      </Typography>
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
