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
import { ModuleEntry } from "../components/ModuleEntry";

export default function CourseRoute() {
  const { id } = useParams();

  const { data } = useSWR(["course", id ?? ""], ([_, id]) => course(id));

  if (data === null) {
    return <SignOut />;
  }

  return (
    <main className="container">
      <h1 className="text-center">Veranstaltung</h1>
      {data && (
        <>
          <TucanUrlLink data={data} />
          <h1 className="text-center">
            {data.inner[0].value.course_id} {data.inner[0].value.title}
          </h1>

          <h2 className="text-center">Module</h2>
          <div className="list-group">
            {data.inner[3].map((module) => {
              return <ModuleEntry key={module.tucan_id} module={module} />;
            })}
          </div>

          <div
            // rome-ignore lint/security/noDangerouslySetInnerHtml: using dompurify
            dangerouslySetInnerHTML={{
              __html: dompurify.sanitize(data.inner[0].value.content),
            }}
          />
          <h2>Termine</h2>
          <button
            className="btn btn-primary mb-2"
            onClick={() => {
              const blob = new Blob([data.inner[4]], { type: "text/calendar" });

              const elem = document.createElement("a");
              elem.setAttribute("href", window.URL.createObjectURL(blob));
              elem.setAttribute("download", `Termine ${data.inner[0].value.title}`);
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
          <h2>Übungsgruppen</h2>
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
