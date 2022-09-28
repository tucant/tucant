import { Chip, TextField } from "@mui/material";
import Alert from "@mui/material/Alert";
import LinearProgress from "@mui/material/LinearProgress";
import List from "@mui/material/List";
import Typography from "@mui/material/Typography";
import dompurify from "dompurify";
import { isLeft } from "fp-ts/lib/Either";
import { keyof } from "io-ts";
import { PathReporter } from "io-ts/lib/PathReporter";
import { useState, useEffect } from "react";
import { RouterLink } from "../MiniDrawer";
import { SearchResultSchema, SearchResultType } from "../validation-io-ts";

export default function SearchCourses() {
  const [data, setData] = useState<SearchResultType | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const [form, setForm] = useState({
    q: "",
  });

  const handleInputChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const target = event.target;
    const value = target.value;
    const AllowedNames = keyof({
      q: null,
    });
    const name = AllowedNames.decode(target.name);
    if (isLeft(name)) {
      throw new Error(
        `Internal Error: Invalid data format in response ${PathReporter.report(
          name
        ).join("\n")}`
      );
    }

    setForm({
      ...form,
      [name.right]: value,
    } as any);
  };

  useEffect(() => {
    const getData = async () => {
      try {
        setLoading(true);
        setError(null);
        const response = await fetch(
          // TODO FIXME url injection
          `http://localhost:8080/search-course?q=${form.q}`,
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
        const actualData = SearchResultSchema.decode(await response.json());
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
  }, [form]);

  return (
    <>
      <Typography variant="h2">Veranstaltungssuche</Typography>
      <TextField
        name="q"
        onChange={handleInputChange}
        value={form.q}
        id="standard-basic"
        label="Suche"
        variant="standard"
        margin="normal"
      />
      {loading && <LinearProgress />}
      {error && <Alert severity="error">{error}</Alert>}

      <p>
        The following syntax is supported:
        <ul>
          <li>
            unquoted text: text not inside quote marks means all words need to
            occur in the document
          </li>
          <li>
            &quot;quoted text&quot;: text inside quote marks means the words
            need to be in the document in that order
          </li>
          <li>
            OR: the word “or” means one of the words needs to occur in the
            document
          </li>
          <li>
            -: a dash means a word is not allowed to be contained in the
            document
          </li>
        </ul>
      </p>

      <List>
        {data != null &&
          data.map((e) => (
            <RouterLink
              key={e[0]}
              to={`/course/${e[0]}`}
              text={
                <span>
                  <Chip label={e[3].toFixed(3)} /> {e[1]}
                </span>
              }
              secondary_text={
                <span
                  dangerouslySetInnerHTML={{ __html: dompurify.sanitize(e[2]) }}
                ></span>
              }
            ></RouterLink>
          ))}
      </List>
    </>
  );
}
