// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import Alert from "@mui/material/Alert";
import LinearProgress from "@mui/material/LinearProgress";
import Typography from "@mui/material/Typography";
import { useState, useEffect } from "react";
import { useLocation, useParams } from "react-router-dom";
import dompurify from "dompurify";
import { course, Course } from "../api";

export default function Course() {
  // TODO refactor into Hook
  const [data, setData] = useState<Course | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const location = useLocation();
  const { id } = useParams();

  useEffect(() => {
    const getData = async () => {
      if (!id) {
        throw new Error("Veranstaltungsnummer fehlt!");
      }
      setData(await course(id));
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
