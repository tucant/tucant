// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import { useParams } from "react-router-dom";
import dompurify from "dompurify";
import { course } from "../api";
import useSWR from "swr";
import SignOut from "./Logout";

export default function CourseRoute() {
  const { id } = useParams();

  const { data } = useSWR(["course", id ?? ""], {
    fetcher: (_, id) => course(id),
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
            {data.course_id} {data.title}
          </h3>
          <div
            dangerouslySetInnerHTML={{
              __html: dompurify.sanitize(data.content),
            }}
          ></div>
        </>
      )}
    </main>
  );
}
