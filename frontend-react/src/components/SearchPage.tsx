// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import dompurify from "dompurify";
import { useState, startTransition } from "react";
import useSWR from "swr";
import { SearchResult } from "../api";
import { Link } from "../Navigation";
import InitialFetch from "../routes/InitialFetch";
import SignOut from "../routes/Logout";

export default function SearchPage(props: {
  title: string;
  function: (input: string) => Promise<SearchResult[]>;
  base_path: string;
}) {
  const [form, setForm] = useState({
    q: "",
  });

  const handleInputChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const target = event.target;
    const value = target.value;

    if (target.name != "q") {
      throw new Error("unexpected input name");
    }
    startTransition(() => {
      setForm({
        ...form,
        [target.name]: value,
      });
    });
  };

  const { data } = useSWR([`search_${props.base_path}`, form.q], {
    fetcher: (_, q) => props.function(q),
  });

  if (data === null) {
    return <SignOut />;
  }

  return (
    <main className="container">
      <h1 className="text-center">{props.title}</h1>
      <form>
        <div className="mb-3">
          <label htmlFor="searchInput" className="form-label">
            Suchbegriff
          </label>
          <input
            name="q"
            onChange={handleInputChange}
            value={form.q}
            type="text"
            placeholder="Suche"
            className="form-control"
            id="searchInput"
            aria-describedby="searchHelp"
          />
          <div id="searchHelp" className="form-text">
            The following syntax is supported:
            <ul>
              <li>
                unquoted text: text not inside quote marks means all words need
                to occur in the document
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
          </div>
        </div>
      </form>

      <div className="list-group">
        {data != null &&
          data.map((e) => (
            <Link
              key={e.tucan_id}
              className="list-group-item list-group-item-action"
              to={`/${props.base_path}/${e.tucan_id}`}
            >
              <div className="d-flex w-100 justify-content-between align-items-center">
                <h5 className="mb-1">{e.title}</h5>
                <span className="badge bg-primary rounded-pill">
                  {e.rank.toFixed(3)}
                </span>
              </div>
              <p
                className="mb-1"
                dangerouslySetInnerHTML={{
                  __html: dompurify.sanitize(e.excerpt),
                }}
              ></p>
            </Link>
          ))}
      </div>
      <InitialFetch />
    </main>
  );
}
