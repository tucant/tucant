import { Button, Chip } from "@mui/material";
import Alert from "@mui/material/Alert";
import LinearProgress from "@mui/material/LinearProgress";
import List from "@mui/material/List";
import Typography from "@mui/material/Typography";
import { useState, useEffect } from "react";
import { useLocation } from "react-router-dom";
import { RouterLink } from "../MiniDrawer";
import InitialFetch from "./InitialFetch";
import dompurify from "dompurify";

export default function Module() {
  const [data, setData] = useState<any>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const location = useLocation();

  useEffect(() => {
    const getData = async () => {
      try {
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
        const actualData = await response.json();
        setData(actualData);
        setError(null);
      } catch (err) {
        setError(String(err));
        setData(null);
      } finally {
        setLoading(false);
      }
    };
    getData();
  }, [location]);

  return (
    <>
      <Typography variant="h2">Modul</Typography>
      {loading && <LinearProgress />}
      {error && <Alert severity="error">{error}</Alert>}

      {data && (
        <>
          <Typography variant="h3">
            {data.module_id} {data.title}
          </Typography>
          <Chip label={`${data.credits} Credits`} />
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
