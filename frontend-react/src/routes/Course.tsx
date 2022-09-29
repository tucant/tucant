// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import Alert from "@mui/material/Alert";
import LinearProgress from "@mui/material/LinearProgress";
import Typography from "@mui/material/Typography";
import { useState, useEffect } from "react";
import { useLocation } from "react-router-dom";
import dompurify from "dompurify";
import { CourseSchema, CourseType } from "../validation-io-ts";
import { PathReporter } from "io-ts/PathReporter";
import { isLeft } from "fp-ts/lib/Either";

export default function Course() {
  // TODO refactor into Hook
  const [data, setData] = useState<CourseType | null>(null);
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
      const actualData = CourseSchema.decode(await response.json());
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
      <Typography variant="h2">Veranstaltung</Typography>
      {loading && <LinearProgress />}
      {error && <Alert severity="error">{error}</Alert>}

      {data && (
        <>
          <Typography variant="h3">
            {data.course_id} {data.title}
          </Typography>
          <div
            dangerouslySetInnerHTML={{
              __html: dompurify.sanitize(data.content),
            }}
          ></div>
        </>
      )}
    </>
  );
}
