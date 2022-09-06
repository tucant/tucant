import { Button } from "@mui/material";
import Alert from "@mui/material/Alert";
import LinearProgress from "@mui/material/LinearProgress";
import List from "@mui/material/List";
import Typography from "@mui/material/Typography";
import { useState, useEffect } from "react";
import { RouterLink } from "../MiniDrawer";
import InitialFetch from "./InitialFetch";

export default function Registration() {
  const [data, setData] = useState<any>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string|null>(null);

  useEffect(() => {
    const getData = async () => {
      try {
        const response = await fetch(
          `http://localhost:8080${window.location.pathname}`,
          {
            credentials: "include"
          }
        );
        if (!response.ok) {
          throw new Error(
            `This is an HTTP error: The status is ${response.status}. ${await response.text()}`
          );
        }
        let actualData = await response.json();
        setData(actualData);
        setError(null);
      } catch(err) {
        setError(String(err));
        setData(null);
      } finally {
        setLoading(false);
      }  
    }
    getData()
  }, [])

  return (
    <>
      <InitialFetch></InitialFetch>

      <Typography variant="h2">
        Module
      </Typography>
      {loading && <LinearProgress />}
      {error && <Alert severity="error">{error}</Alert>}
      <List>
        { data != null && data.map((e: [string, string]) => <RouterLink to={`/modules/${e[1]}/`} text={e[0]}></RouterLink>)


        }
      </List>
    </>
  );
}
