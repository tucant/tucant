// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import { useParams } from "react-router-dom";
import dompurify from "dompurify";
import { course } from "../api";
import useSWR from "swr";
import SignOut from "./Logout";
import { TucanUrlLink } from "../components/TucanUrlLink";
import { Link } from "../Navigation";

export default function CourseRoute() {
  const { id } = useParams();

  const { data } = useSWR(["course", id ?? ""], {
    fetcher: ([_, id]) => course(id),
  });

  if (data === null) {
    return <SignOut />;
  }

  return (
    <main className="container">
      <h1 className="text-center">Veranstaltung</h1>
      {data && (
        <>
          <h3 className="text-center">
            {data.inner[0].course_id} {data.inner[0].title}
          </h3>
          <TucanUrlLink data={data} />
          <div
            // rome-ignore lint/security/noDangerouslySetInnerHtml: using dompurify
            dangerouslySetInnerHTML={{
              __html: dompurify.sanitize(data.inner[0].content),
            }}
          />
          <h4>Termine</h4>
          <div className="list-group">
            {data.inner[2].map((e) => (
              <div
                key={e.timestamp_start + e.timestamp_end + e.room}
                className="list-group-item list-group-item-action"
              >
                von {new Date(e.timestamp_start).toLocaleString()} bis{" "}
                {new Date(e.timestamp_end).toLocaleString()} in {e.room} bei{" "}
                {e.teachers}
              </div>
            ))}
          </div>
          <h4>Übungsgruppen</h4>
          <div className="list-group">
            {data.inner[1].map((e) => (
              <Link
                key={e.tucan_id}
                className="list-group-item list-group-item-action"
                to={`/course-group/${e.tucan_id}`}
              >
                {e.title}
              </Link>
            ))}
          </div>
        </>
      )}
    </main>
  );
}
