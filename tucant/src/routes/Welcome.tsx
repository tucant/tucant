import Alert from "@mui/material/Alert";
import LinearProgress from "@mui/material/LinearProgress";
import { isLeft } from "fp-ts/lib/Either";
import { PathReporter } from "io-ts/lib/PathReporter";
import { useState, useEffect } from "react";
import { WelcomeSchema } from "../validation-io-ts";

export default function Welcome() {
  const [data, setData] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const getData = async () => {
      try {
        const response = await fetch("http://localhost:8080", {
          credentials: "include",
        });
        if (!response.ok) {
          throw new Error(
            `This is an HTTP error: The status is ${response.status}`
          );
        }
        const actualData = WelcomeSchema.decode(await response.json());
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
  }, []);

  return (
    <div className="App">
      <h1>Welcome</h1>
      {loading && <LinearProgress />}
      {error && <Alert severity="error">{error}</Alert>}
      <ul>{JSON.stringify(data)}</ul>
    </div>
  );
}
