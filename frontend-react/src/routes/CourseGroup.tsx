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

  const { data } = useSWR(["course-group", id ?? ""], {
    fetcher: ([_, id]) => course_group(id),
  });

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
        </>
      )}
    </main>
  );
}
