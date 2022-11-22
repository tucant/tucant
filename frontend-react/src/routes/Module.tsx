// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import { useParams } from "react-router-dom";
import dompurify from "dompurify";
import { module } from "../api";
import useSWR from "swr";
import { Link } from "../Navigation";
import SignOut from "./Logout";

export default function Module() {
  const { id } = useParams();

  const { data } = useSWR(["course", id ?? ""], {
    fetcher: (_, id) => module(id),
  });

  if (data === null) {
    return <SignOut />;
  }

  return (
    <main className="container">
      {data && (
        <>
          <h1>
            {data.module.module_id} {data.module.title}
          </h1>
          {data.path.map((p, i) => (
            <nav key={i} aria-label="breadcrumb">
              <ol className="breadcrumb">
                {p.map((pe) => (
                  <li key={pe.tucan_id} className="breadcrumb-item">
                    <Link to={`/modules/${pe.tucan_id}`}>{pe.name}</Link>
                  </li>
                ))}
              </ol>
            </nav>
          ))}
          <div className="grid" style={{ "--bs-columns": 2 }}>
            <div className="g-col-1">
              <pre>
                <code className="language-html" data-lang="html">
                  This is some test code
                </code>
              </pre>
            </div>
            <div className="g-col-1" style={{ justifySelf: "end" }}>
              <button type="button" className="btn" title="">
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  width="16"
                  height="16"
                  fill="currentColor"
                  className="bi bi-clipboard"
                  viewBox="0 0 16 16"
                >
                  <path d="M4 1.5H3a2 2 0 0 0-2 2V14a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2V3.5a2 2 0 0 0-2-2h-1v1h1a1 1 0 0 1 1 1V14a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1V3.5a1 1 0 0 1 1-1h1v-1z" />
                  <path d="M9.5 1a.5.5 0 0 1 .5.5v1a.5.5 0 0 1-.5.5h-3a.5.5 0 0 1-.5-.5v-1a.5.5 0 0 1 .5-.5h3zm-3-1A1.5 1.5 0 0 0 5 1.5v1A1.5 1.5 0 0 0 6.5 4h3A1.5 1.5 0 0 0 11 2.5v-1A1.5 1.5 0 0 0 9.5 0h-3z" />
                </svg>
              </button>
            </div>
          </div>
          <span className="badge rounded-pill text-bg-primary">{`${
            data.module.credits ?? 0
          } Credits`}</span>
          <div
            dangerouslySetInnerHTML={{
              __html: dompurify.sanitize(data.module.content),
            }}
          ></div>
        </>
      )}
    </main>
  );
}
