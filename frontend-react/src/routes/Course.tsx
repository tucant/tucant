// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import { useState, useEffect } from "react";
import { useLocation, useParams } from "react-router-dom";
import dompurify from "dompurify";
import { course, Course } from "../api";
import useSWR from "swr";

export default function CourseRoute() {
  const { id } = useParams();

  const { data, error } = useSWR(["course", id ?? ""], {
    fetcher: (_, id) => course(id),
  });

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
