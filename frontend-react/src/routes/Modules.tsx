// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import { Breadcrumbs, Link } from "@mui/material";
import Alert from "@mui/material/Alert";
import LinearProgress from "@mui/material/LinearProgress";
import Typography from "@mui/material/Typography";
import { useState, useEffect } from "react";
import { useLocation } from "react-router-dom";
import InitialFetch from "./InitialFetch";
import NavigateNextIcon from "@mui/icons-material/NavigateNext";
import {
  ModulesResponseSchema,
  ModulesResponseType,
} from "../validation-io-ts";
import { isLeft } from "fp-ts/lib/Either";
import { PathReporter } from "io-ts/lib/PathReporter";
import {ModuleList} from "../components/ModuleList";

export default function Modules() {
  const location = useLocation();

  const [data, setData] = useState<ModulesResponseType | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const getData = async () => {
      const response = await fetch(
        `http://localhost:8080${location.pathname}`,
        { credentials: "include" }
      );
      if (!response.ok) {
        throw new Error(
          `This is an HTTP error: The status is ${
            response.status
          }. ${await response.text()}`
        );
      }
      const actualData = ModulesResponseSchema.decode(await response.json());
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

  const breadcrumbs = [
    <Link underline="hover" key="1" color="inherit" href="/">
      MUI
    </Link>,
    <Link
      underline="hover"
      key="2"
      color="inherit"
      href="/material-ui/getting-started/installation/"
    >
      Core
    </Link>,
    <Typography key="3" color="text.primary">
      Breadcrumb
    </Typography>,
  ];

  return (
    <>
      <Breadcrumbs
        separator={<NavigateNextIcon fontSize="small" />}
        aria-label="breadcrumb"
      >
        {breadcrumbs}
      </Breadcrumbs>

      <Typography variant="h2">Module</Typography>
      {loading && <LinearProgress />}
      {error && <Alert severity="error">{error}</Alert>}
      {data && <ModuleList listData={data} />}

      <InitialFetch></InitialFetch>
    </>
  );
}
