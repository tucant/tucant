import { Button } from "@mui/material";
import Alert from "@mui/material/Alert";
import LinearProgress from "@mui/material/LinearProgress";
import List from "@mui/material/List";
import Typography from "@mui/material/Typography";
import { useState, useEffect } from "react";
import { useLocation } from "react-router-dom";
import { RouterLink } from "../MiniDrawer";
import InitialFetch from "./InitialFetch";
import Module from "./Module";

export default function Modules() {
  const location = useLocation();

  const [data, setData] = useState<any>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

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
        let actualData = await response.json();
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
      <InitialFetch></InitialFetch>

      <Typography variant="h2">Module</Typography>
      {loading && <LinearProgress />}
      {error && <Alert severity="error">{error}</Alert>}
      <List>
        {data != null && "Menus" in data &&
          data.Menus.map((e: { normalized_name: string; name: string; }) => (
            <RouterLink
              to={`${location.pathname}${e.normalized_name}/`}
              text={e.name}
            ></RouterLink>
          ))}
          {data != null && "Modules" in data &&
          data.Modules.map((e: [string, string]) => (
            <RouterLink
              to={`${location.pathname}${e[1]}`}
              text={e[0]}
            ></RouterLink>
          ))}
      </List>
    </>
  );
}
