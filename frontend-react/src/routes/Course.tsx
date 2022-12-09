// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import { useParams } from "react-router-dom";
import dompurify from "dompurify";
import { course } from "../api";
import useSWR from "swr";
import SignOut from "./Logout";
import { TucanUrlLink } from "../components/TucanUrlLink";

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
            {data.inner.course_id} {data.inner.title}
          </h3>
          <TucanUrlLink data={data} />
          <div
            dangerouslySetInnerHTML={{
              __html: dompurify.sanitize(data.inner.content),
            }}
          ></div>
        </>
      )}
    </main>
  );
}
