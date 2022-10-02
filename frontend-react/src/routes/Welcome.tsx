// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import Alert from "@mui/material/Alert";
import LinearProgress from "@mui/material/LinearProgress";
import { useState, useEffect } from "react";
import { index } from "../api";

export default function Welcome() {
  const [data, setData] = useState<string | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const getData = async () => {
      setData(await index(null));
      setError(null);
    };
    getData()
      .catch((err) => {
        setError(String(err));
        setData(null);
      })
      .finally(() => {
        setLoading(false);
      });
  }, []);

  return (
    <div className="App">
      <h1>Welcome</h1>
      {loading && <LinearProgress />}
      {error && <Alert severity="error">{error}</Alert>}
      <ul>{JSON.stringify(data)}</ul>
    </div>
  );
}
