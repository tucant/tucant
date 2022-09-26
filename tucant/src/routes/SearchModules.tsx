import { Button, Chip, TextField } from "@mui/material";
import Alert from "@mui/material/Alert";
import LinearProgress from "@mui/material/LinearProgress";
import List from "@mui/material/List";
import Typography from "@mui/material/Typography";
import dompurify from "dompurify";
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
        setLoading(true);
        setError(null);
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
      <TextField name="q" onChange={handleInputChange} value={form.q} id="standard-basic" label="Suche" variant="standard" margin="normal" />
      {loading && <LinearProgress />}
      {error && <Alert severity="error">{error}</Alert>}

      <p>
        The following syntax is supported:
        <ul>
          <li>unquoted text: text not inside quote marks means all words need to occur in the document</li>
          <li>"quoted text": text inside quote marks means the words need to be in the document in that order</li>
          <li>OR: the word “or” means one of the words needs to occur in the document</li>
          <li>-: a dash means a word is not allowed to be contained in the document</li>
        </ul>
      </p>

      <List>
        {data != null &&
          data.map((e: [number, string, string, number]) => (
            <RouterLink
              to={`/module/${e[0]}`}
              text={<span><Chip label={e[3].toFixed(3)} /> {e[1]}</span>}
              secondary_text={<span
                dangerouslySetInnerHTML={{ __html: dompurify.sanitize(e[2]) }}
              ></span>}
            ></RouterLink>
          ))}
      </List>
    </>
  );
}
