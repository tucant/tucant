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
            dangerouslySetInnerHTML={{
              __html: dompurify.sanitize(data.inner[0].content),
            }}
          ></div>
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
