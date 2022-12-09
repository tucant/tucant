// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import useSWR from "swr";
import { my_exams } from "../api";
import { TucanUrlLink } from "../components/TucanUrlLink";
import { Link } from "../Navigation";
import SignOut from "./Logout";

export default function MyExams() {
  const { data } = useSWR("my-exams", {
    fetcher: () => my_exams(null),
  });

  if (data === null) {
    return <SignOut />;
  }

  return (
    <div className="container">
      <h1 className="text-center">Meine Prüfungen</h1>
      <div className="list-group">
        {data != null && (
          <>
            <TucanUrlLink data={data} />
            {data.inner.map((e) => (
              <Link
                key={e.tucan_id}
                className="list-group-item list-group-item-action"
                to={`/exam/${e.tucan_id}`}
              >
                {e.tucan_id}
              </Link>
            ))}
          </>
        )}
      </div>
    </div>
  );
}
