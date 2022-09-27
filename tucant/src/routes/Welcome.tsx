import Alert from "@mui/material/Alert";
import LinearProgress from "@mui/material/LinearProgress";
import { useState, useEffect } from "react";

export default function Welcome() {
  const [data, setData] = useState(null);
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
