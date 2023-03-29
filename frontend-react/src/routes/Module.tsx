// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import { useParams } from "react-router-dom";
import dompurify from "dompurify";
import { module } from "../api";
import useSWR from "swr";
import { Link } from "../Navigation";
import SignOut from "./Logout";
import { TucanUrlLink } from "../components/TucanUrlLink";

export default function Module() {
  const { id } = useParams();

  const { data } = useSWR(["module", id ?? ""], ([_, id]) => module(id));

  if (data === null) {
    return <SignOut />;
  }

  return (
    <main className="container">
      {data && (
        <>
          <h1>
            {data.inner.module.module_id} {data.inner.module.title}
          </h1>
          {data.inner.path.map((p) => (
            <nav
              key={p.map((pe) => pe.tucan_id).join()}
              aria-label="breadcrumb"
            >
              <ol className="breadcrumb">
                {p.map((pe) => (
                  <li key={pe.tucan_id} className="breadcrumb-item">
                    <Link to={`/modules/${pe.tucan_id}`}>{pe.name}</Link>
                  </li>
                ))}
              </ol>
            </nav>
          ))}
          <span className="badge rounded-pill text-bg-primary">{`${data.inner.module.credits ?? 0
            } Credits`}</span>
          <TucanUrlLink data={data} />

          <div
            // rome-ignore lint/security/noDangerouslySetInnerHtml: using dompurify
            dangerouslySetInnerHTML={{
              __html: dompurify.sanitize(data.inner.module.content),
            }}
          />
        </>
      )}
    </main>
  );
}
