// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import { useLayoutEffect, useRef, useState } from "react";

type buttonVariantType = "primary" | "warning" | "success";

export default function InitialFetch() {
  const [data, setData] = useState<string>("");
  const [isLoading, setLoading] = useState(false);
  const [buttonVariant, setButtonVariant] =
    useState<buttonVariantType>("primary");
  const setSuccess = (success: boolean | null) => {
    switch (success) {
      case null:
        setButtonVariant("primary");
        break;
      case false:
        setButtonVariant("warning");
        break;
      case true:
        setButtonVariant("success");
        break;
      default:
        setButtonVariant("primary");
        break;
    }
  };
  const [error, setError] = useState<string | null>(null);
  const scrollEl = useRef<HTMLPreElement>(null);
  const [everStarted, setEverStarted] = useState(false);

  useLayoutEffect(() => {
    if (scrollEl.current) {
      scrollEl.current.scrollTop = scrollEl.current.scrollHeight;
    }
  }, [data]);

  return (
    <>
      {error && (
        <div className="mt-3 alert alert-danger" role="alert">
          {error}
        </div>
      )}
      <button
        className={`my-3 btn btn-${buttonVariant}`}
        type="submit"
        disabled={isLoading}
        onClick={() => {
          (async () => {
            setEverStarted(true);
            setSuccess(null);
            setError(null);
            setLoading(true);
            setData("Synchronisierung wird gestartet.");

            const response = await fetch("http://localhost:8080/setup", {
              credentials: "include",
              method: "POST",
              headers: {
                "Content-Type": "application/json",
                "x-csrf-protection": "tucant",
              },
              body: "null",
            });

            const reader = response.body?.getReader();
            let value: ReadableStreamReadResult<Uint8Array> | undefined;
            while (!(value = await reader?.read())?.done) {
              setData((data) => data + new TextDecoder().decode(value?.value));
            }
            setData((data) => data + "\nFertig");
            setSuccess(true);
          })().catch((error) => {
            setError(String(error));
            setSuccess(false);
          });
        }}
      >
        {isLoading && (
          <span
            className="spinner-border spinner-border-sm"
            role="status"
            aria-hidden="true"
          />
        )}{" "}
        Initiale Synchronisierung
      </button>
      {everStarted && (
        <div className="accordion mb-3" id="accordionExample">
          <div className="accordion-item">
            <h2 className="accordion-header" id="headingOne">
              <button
                className="accordion-button"
                type="button"
                data-bs-toggle="collapse"
                data-bs-target="#collapseOne"
                aria-expanded="true"
                aria-controls="collapseOne"
              >
                {data.substring(data.lastIndexOf("\n"))}
              </button>
            </h2>
            <div
              id="collapseOne"
              className="accordion-collapse collapse show"
              aria-labelledby="headingOne"
              data-bs-parent="#accordionExample"
            >
              <div className="accordion-body">
                <pre ref={scrollEl} style={{ maxHeight: "25vh" }}>
                  {data}
                </pre>
              </div>
            </div>
          </div>
        </div>
      )}
    </>
  );
}
