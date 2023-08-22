// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import { Link } from "../Navigation";
import { my_courses } from "../api";
import { TucanUrlLink } from "../components/TucanUrlLink";
import SignOut from "./Logout";
import useSWR from "swr";

export default function MyCourses() {
  const { data } = useSWR("my-courses", () => my_courses(null));

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
            {data.inner[0].map((c) => (
              <Link
                key={c.value.tucan_id}
                className="list-group-item list-group-item-action"
                to={`/course/${c.value.tucan_id}`}
              >
                {c.value.title}
              </Link>
            ))}
            {data.inner[1].map((c) => (
              <Link
                key={c.tucan_id}
                className="list-group-item list-group-item-action"
                to={`/course-group/${c.tucan_id}`}
              >
                {c.title}
              </Link>
            ))}
          </>
        )}
      </div>
    </div>
  );
}
