// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import useSWR from "swr";
import { my_courses } from "../api";
import { CourseOrCourseGroupEntry } from "../components/CourseOrCourseGroupEntry";
import { TucanUrlLink } from "../components/TucanUrlLink";
import SignOut from "./Logout";

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
            {data.inner.map((e) => (
              <CourseOrCourseGroupEntry key={e.value[0].tucan_id} c={e} />
            ))}
          </>
        )}
      </div>
    </div>
  );
}
