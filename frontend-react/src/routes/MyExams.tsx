// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import useSWR from "swr";
import { my_exams } from "../api";
import { formatLocalDate } from "../api_base";
import { TucanUrlLink } from "../components/TucanUrlLink";
import { Link } from "../Navigation";
import SignOut from "./Logout";

export default function MyExams() {
  const { data } = useSWR("my-exams", () => my_exams(null));

  if (data === null) {
    return <SignOut />;
  }

  return (
    <div className="container">
      <h1 className="text-center">Meine Prüfungen</h1>
      {data != null && (
        <>
          <TucanUrlLink data={data} />
          <div className="list-group">
            {data.inner[0].map((e) => (
              <Link
                key={e[1].tucan_id}
                className="list-group-item list-group-item-action"
                to={`/exam/${e[1].tucan_id}`}
              >
                {e[0].title} ({e[1].exam_type}){" "}
                {formatLocalDate(e[1].exam_time_start)
                  ?.concat("-")
                  .concat(formatLocalDate(e[1].exam_time_end) ?? "")}
              </Link>
            ))}
            {data.inner[1].map((e) => (
              <Link
                key={e[1].tucan_id}
                className="list-group-item list-group-item-action"
                to={`/exam/${e[1].tucan_id}`}
              >
                {e[0].value.title} ({e[1].exam_type}){" "}
                {formatLocalDate(e[1].exam_time_start)
                  ?.concat("-")
                  .concat(formatLocalDate(e[1].exam_time_end) ?? "")}
              </Link>
            ))}
          </div>
        </>
      )}
    </div>
  );
}
