// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import { useParams } from "react-router-dom";
import { exam } from "../api";
import useSWR from "swr";
import SignOut from "./Logout";
import { TucanUrlLink } from "../components/TucanUrlLink";
import { Link } from "../Navigation";
import { formatLocalDate } from "../api_base";

export default function Exam() {
  const { id } = useParams();

  const { data } = useSWR(["exam", id ?? ""], ([_, id]) => exam(id));

  if (data === null) {
    return <SignOut />;
  }

  return (
    <main className="container">
      <h1 className="text-center">Prüfung</h1>
      {data && (
        <>
          <h3 className="text-center">{data.inner[0].exam_type}</h3>
          <TucanUrlLink data={data} />
          Zeitraum:{" "}
          {formatLocalDate(data.inner[0].exam_time_start)
            ?.concat(" - ")
            .concat(formatLocalDate(data.inner[0].exam_time_end) ?? "")}
          <br />
          Anmeldezeitraum:{" "}
          {formatLocalDate(data.inner[0].registration_start)
            ?.concat(" - ")
            .concat(formatLocalDate(data.inner[0].registration_end) ?? "")}
          <br />
          Abmeldezeitraum:{" "}
          {formatLocalDate(data.inner[0].unregistration_start)
            ?.concat(" - ")
            .concat(formatLocalDate(data.inner[0].unregistration_end) ?? "")}
          <br />
          Prüfende Person: {data.inner[0].examinator}
          <br />
          Raum: {data.inner[0].room}
          <br />
          Semester: {data.inner[0].semester}
          <br />
          <h4 className="text-center">Enthalten in:</h4>
          <div className="list-group">
            {data.inner[1].map((e) => (
              <Link
                key={e.tucan_id}
                className="list-group-item list-group-item-action"
                to={`/module/${e.tucan_id}`}
              >
                {e.title}
              </Link>
            ))}
            {data.inner[2].map((e) => (
              <Link
                key={e.value.tucan_id}
                className="list-group-item list-group-item-action"
                to={`/course/${e.value.tucan_id}`}
              >
                {e.value.title}
              </Link>
            ))}
          </div>
        </>
      )}
    </main>
  );
}
