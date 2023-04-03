// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import { useParams } from "react-router-dom";
import useSWR from "swr";
import { courses } from "../api";
import { ModuleList } from "../components/ModuleList";
import { TucanUrlLink } from "../components/TucanUrlLink";
import { Link } from "../Navigation";
import SignOut from "./Logout";
import { CourseEntry } from "../components/CourseEntry";

export default function Courses() {
  const { id } = useParams();

  const { data } = useSWR(["courses", id ?? null], ([_, id]) =>
    courses(id),
  );

  if (data === null) {
    return <SignOut />;
  }

  return (
    <main className="container">
      <h1 className="text-center">{data?.inner[0].name}</h1>
      {data && (
        <>
          <div className="list-group">
            {data.inner[1].map((p) => (
              <Link
                key={p.tucan_id}
                className="list-group-item list-group-item-action"
                to={`/vv/${p.tucan_id}`}
              >
                {p.name}
              </Link>
            ))}
          </div>
          <TucanUrlLink data={data} />
          <div className="list-group">
            {data.inner[2].map(course =>
              <CourseEntry key={course.tucan_id} c={course} />
            )}
          </div>
        </>
      )}
    </main>
  );
}
