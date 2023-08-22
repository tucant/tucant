// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import { Link } from "../Navigation";
import { module } from "../api";
import { CourseEntry } from "../components/CourseEntry";
import { TucanUrlLink } from "../components/TucanUrlLink";
import SignOut from "./Logout";
import dompurify from "dompurify";
import { useParams } from "react-router-dom";
import useSWR from "swr";

export default function Module() {
  const { id } = useParams();

  const { data } = useSWR(["module", id ?? ""], ([_, id]) => module(id));

  if (data === null) {
    return <SignOut />;
  }

  return (
    <main className="container">
      <h1 className="text-center">Modul</h1>
      {data && (
        <>
          <TucanUrlLink data={data} />
          <h1 className="text-center">
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
          <span className="badge rounded-pill text-bg-primary">{`${
            data.inner.module.credits ?? 0
          } Credits`}</span>

          <h2 className="text-center">Veranstaltungen</h2>
          <div className="list-group">
            {data.inner.courses.map((course) => {
              return <CourseEntry key={course.value.tucan_id} c={course} />;
            })}
          </div>

          <h2 className="text-center">Prüfungen</h2>
          <div className="list-group">
            {data.inner.exam_types.map((examType) => {
              return (
                <Link
                  key={examType.exam_type}
                  className="list-group-item list-group-item-action"
                  to="/not-implemented"
                >
                  {examType.exam_type}
                </Link>
              );
            })}
          </div>

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
