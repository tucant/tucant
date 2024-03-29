// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import { course_group } from "../api";
import { TucanUrlLink } from "../components/TucanUrlLink";
import SignOut from "./Logout";
import { useParams } from "react-router-dom";
import useSWR from "swr";

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
          <button
            type="button"
            className="btn btn-primary mb-2"
            onClick={() => {
              const blob = new Blob([data.inner[3]], { type: "text/calendar" });

              const elem = document.createElement("a");
              elem.setAttribute("href", window.URL.createObjectURL(blob));
              elem.setAttribute("download", `Termine ${data.inner[1].title}`);
              elem.style.display = "none";
              document.body.appendChild(elem);
              elem.click();
              document.body.removeChild(elem);
            }}
          >
            Download
          </button>
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
