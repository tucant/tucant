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
            {data.inner.map((e) => (
              <Link
                key={e.value.tucan_id}
                className="list-group-item list-group-item-action"
                to={`/${e.type == "Course" ? "course" : "course-group"}/${
                  e.value.tucan_id
                }`}
              >
                {e.value.title}
              </Link>
            ))}
          </>
        )}
      </div>
    </div>
  );
}
