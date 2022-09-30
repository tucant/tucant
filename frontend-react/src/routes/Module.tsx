// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import { Breadcrumbs, Chip, Link } from "@mui/material";
import Alert from "@mui/material/Alert";
import LinearProgress from "@mui/material/LinearProgress";
import Typography from "@mui/material/Typography";
import { useState, useEffect } from "react";
import { useLocation } from "react-router-dom";
import dompurify from "dompurify";
import { ModuleResponse } from "../validation-io-ts";
import { PathReporter } from "io-ts/PathReporter";
import { isLeft } from "fp-ts/lib/Either";
import { TypeOf } from "io-ts";
import NavigateNextIcon from "@mui/icons-material/NavigateNext";

export default function Module() {
  const [data, setData] = useState<TypeOf<typeof ModuleResponse> | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const location = useLocation();

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
      const actualData = ModuleResponse.decode(await response.json());
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
      {loading && <LinearProgress />}
      {error && <Alert severity="error">{error}</Alert>}

      {data && (
        <>
          <Typography variant="h4" component="h1">
            {data.module.module_id} {data.module.title}
          </Typography>
          {data.path.map((p, i) => (
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
          <Chip label={`${data.module.credits ?? 0} Credits`} />
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
