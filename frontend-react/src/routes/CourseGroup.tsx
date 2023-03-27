// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import { useParams } from "react-router-dom";
import { course_group } from "../api";
import useSWR from "swr";
import SignOut from "./Logout";
import { TucanUrlLink } from "../components/TucanUrlLink";

export default function CourseGroupRoute() {
  const { id } = useParams();

  const { data } = useSWR(["course-group", id ?? ""], ([_, id]) =>
    course_group(id),
  );

  if (data === null) {
    return <SignOut />;
  }

  return (
    <main className="container">
      <h1 className="text-center">Übungsgruppe</h1>
      {data && (
        <>
          <h3 className="text-center">
            {data.inner[0].course_id} {data.inner[1].title}
          </h3>
          <TucanUrlLink data={data} />
          <a href={`/course/${data.inner[1].course}`}>
            zugehörige Veranstaltung
          </a>
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
        </>
      )}
    </main>
  );
}
