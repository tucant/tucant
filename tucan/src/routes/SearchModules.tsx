import { Button, TextField } from "@mui/material";
import Alert from "@mui/material/Alert";
import LinearProgress from "@mui/material/LinearProgress";
import List from "@mui/material/List";
import Typography from "@mui/material/Typography";
import { useState, useEffect } from "react";
import { useLocation } from "react-router-dom";
import { RouterLink } from "../MiniDrawer";
import InitialFetch from "./InitialFetch";
import Module from "./Module";

export default function SearchModules() {
  const location = useLocation();

  const [data, setData] = useState<any>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const [form, setForm] = useState({
    q: "",
  });

  let handleInputChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const target = event.target;
    const value = target.value;
    const name = target.name;

    setForm({
      ...form,
      [name]: value,
    } as any);
  };

  useEffect(() => {
    const getData = async () => {
      try {
        const response = await fetch(
          // TODO FIXME url injection
          `http://localhost:8080/search-module?q=${form.q}`,
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
  }, [form]);

  return (
    <>
      <Typography variant="h2">Modulsuche</Typography>
      <TextField name="q" onChange={handleInputChange} id="standard-basic" label="Suche" variant="standard" margin="normal" />
      {loading && <LinearProgress />}
      {error && <Alert severity="error">{error}</Alert>}
      <List>
        {data != null &&
          data.map((e: [string, string, string]) => (
            <RouterLink
              to={`${location.pathname}${e[0]}`}
              text={e[0]}
            ></RouterLink>
          ))}
      </List>
    </>
  );
}
