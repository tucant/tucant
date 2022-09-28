import { Chip } from "@mui/material";
import Alert from "@mui/material/Alert";
import LinearProgress from "@mui/material/LinearProgress";
import Typography from "@mui/material/Typography";
import { useState, useEffect } from "react";
import { useLocation } from "react-router-dom";
import dompurify from "dompurify";
import { ModuleSchema, ModuleType } from "../validation-io-ts";
import { PathReporter } from "io-ts/PathReporter";
import { isLeft } from "fp-ts/lib/Either";

export default function Module() {
  const [data, setData] = useState<ModuleType | null>(null);
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
        const actualData = ModuleSchema.decode(await response.json());
        if (isLeft(actualData)) {
          throw new Error(
            `Internal Error: Invalid data format in response ${PathReporter.report(
              actualData
            ).join("\n")}`
          );
        }
        setData(actualData.right);
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
