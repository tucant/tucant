// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import { useParams } from "react-router-dom";
import dompurify from "dompurify";
import { exam } from "../api";
import useSWR from "swr";
import SignOut from "./Logout";
import { TucanUrlLink } from "../components/TucanUrlLink";

export default function Exam() {
  const { id } = useParams();

  const { data } = useSWR(["exam", id ?? ""], {
    fetcher: ([_, id]) => exam(id),
  });

  if (data === null) {
    return <SignOut />;
  }

  return (
    <main className="container">
      <h1 className="text-center">Prüfung</h1>
      {data && (
        <>
          <h3 className="text-center">
            {data.inner.tucan_id} {data.inner.exam_time_start}
          </h3>
          <TucanUrlLink data={data} />
        </>
      )}
    </main>
  );
}
