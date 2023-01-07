// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import useSWR from "swr";
import { my_courses } from "../api";
import { TucanUrlLink } from "../components/TucanUrlLink";
import { Link } from "../Navigation";
import SignOut from "./Logout";

export default function MyCourses() {
  const { data } = useSWR("my-courses", {
    fetcher: () => my_courses(null),
  });

  if (data === null) {
    return <SignOut />;
  }

  return (
    <div className="container">
      <h1 className="text-center">Meine Veranstaltungen</h1>
      <div className="list-group">
        {data != null && (
          <>
            <TucanUrlLink data={data} />
            {data.inner.map((e) =>
              e.type == "Course" ? (
                <Link
                  key={e.value[0].tucan_id}
                  className="list-group-item list-group-item-action"
                  to={`/course/${e.value[0].tucan_id}`}
                >
                  {e.value[0].title}
                </Link>
              ) : (
                <Link
                  key={e.value[0].tucan_id}
                  className="list-group-item list-group-item-action"
                  to={`/course-group/${e.value[0].tucan_id}`}
                >
                  {e.value[0].title}
                </Link>
              ),
            )}
          </>
        )}
      </div>
    </div>
  );
}
