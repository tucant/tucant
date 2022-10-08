// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import { Chip, TextField } from "@mui/material";
import Alert from "@mui/material/Alert";
import LinearProgress from "@mui/material/LinearProgress";
import List from "@mui/material/List";
import Typography from "@mui/material/Typography";
import dompurify from "dompurify";
import { useState, useEffect } from "react";
import { SearchResult, search_course } from "../api";
import { RouterLink } from "../Navigation";

export default function SearchCourses() {
  const [data, setData] = useState<SearchResult[] | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const [form, setForm] = useState({
    q: "",
  });

  const handleInputChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const target = event.target;
    const value = target.value;

    if (target.name != "q") {
      throw new Error("unexpected input name");
    }
    setForm({
      ...form,
      [target.name]: value,
    });
  };

  useEffect(() => {
    const getData = async () => {
      setLoading(true);
      setError(null);
      setData(await search_course(form.q));
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
              key={e.tucan_id}
              to={`/course/${e.tucan_id}`}
              text={
                <span>
                  <Chip label={e.rank.toFixed(3)} /> {e.title}
                </span>
              }
              secondary_text={
                <span
                  dangerouslySetInnerHTML={{
                    __html: dompurify.sanitize(e.excerpt),
                  }}
                ></span>
              }
            ></RouterLink>
          ))}
      </List>
    </>
  );
}
