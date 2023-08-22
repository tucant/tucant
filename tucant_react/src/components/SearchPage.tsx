// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import { Link } from "../Navigation";
import { SearchResult } from "../api";
import InitialFetch from "../routes/InitialFetch";
import SignOut from "../routes/Logout";
import dompurify from "dompurify";
import React, { useState, useTransition } from "react";
import useSWR from "swr";

export default function SearchPage(props: {
  title: string;
  function: (input: string) => Promise<SearchResult[]>;
  base_path: string;
}) {
  const [form, setForm] = useState({
    q: "",
  });

  const [isLoading, startTransition] = useTransition();

  const handleInputChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const target = event.target;
    const value = target.value;

    if (target.name !== "q") {
      throw new Error("unexpected input name");
    }
    startTransition(() => {
      setForm({
        ...form,
        [target.name]: value,
      });
    });
  };

  const { data } = useSWR([`search_${props.base_path}`, form.q], ([_, q]) =>
    props.function(q),
  );

  if (data === null) {
    return <SignOut />;
  }

  return (
    <main className="container">
      <h1 className="text-center">{props.title}</h1>
      <InitialFetch url={"/setup"} />
      <form>
        <div className="mb-3">
          <div className="input-group mb-3">
            <input
              name="q"
              onChange={handleInputChange}
              value={form.q}
              type="text"
              placeholder="Suche"
              className="form-control"
              id="searchInput"
              aria-describedby="searchButton"
            />
            <button
              className="btn btn-outline-secondary"
              disabled={isLoading}
              type="button"
              id="searchButton"
              onClick={() => {
                startTransition(() => {
                  setForm({
                    ...form,
                  });
                });
              }}
            >
              {isLoading && (
                <span
                  className="spinner-border spinner-border-sm"
                  role="status"
                  aria-hidden="true"
                />
              )}{" "}
              Suchen
            </button>
          </div>

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
        {data?.map((e) => (
          <Link
            key={e.tucan_id}
            className="list-group-item list-group-item-action"
            to={`/${props.base_path}/${e.tucan_id}`}
          >
            <div className="d-flex w-100 justify-content-between align-items-center">
              <h5
                className="mb-1"
                // rome-ignore lint/security/noDangerouslySetInnerHtml: using dompurify
                dangerouslySetInnerHTML={{
                  __html: dompurify.sanitize(e.title),
                }}
              />
              <span className="badge bg-primary rounded-pill">
                {e.rank.toFixed(3)}
              </span>
            </div>
            <p
              className="mb-1"
              // rome-ignore lint/security/noDangerouslySetInnerHtml: using dompurify
              dangerouslySetInnerHTML={{
                __html: dompurify.sanitize(e.excerpt),
              }}
            />
          </Link>
        ))}
      </div>
    </main>
  );
}
