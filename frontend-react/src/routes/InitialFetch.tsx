// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import LoadingButton from "@mui/lab/LoadingButton";
import Alert from "@mui/material/Alert";
import { useState } from "react";

export default function InitialFetch() {
  const [data, setData] = useState<string>("");
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  return (
    <>
      {error && <Alert severity="error">{error}</Alert>}
      <LoadingButton
        loading={loading}
        onClick={() => {
          (async () => {
            setError(null);
            setLoading(true);

            const response = await fetch("http://localhost:8080/setup", {
              credentials: "include",
              method: "POST",
              headers: {
                "x-csrf-protection": "tucant",
              },
              body: "",
            });

            const reader = response.body?.getReader();
            let value: ReadableStreamReadResult<Uint8Array> | undefined;
            while (!(value = await reader?.read())?.done) {
              setData(new TextDecoder().decode(value?.value));
            }
          })()
            .catch((error) => {
              setError(String(error));
            })
            .finally(() => {
              setLoading(false);
            });
        }}
      >
        Initial sync
      </LoadingButton>
      {data}
    </>
  );
}
