import LoadingButton from "@mui/lab/LoadingButton";
import Alert from "@mui/material/Alert";
import Button from "@mui/material/Button";
import { useRef, useState } from "react";

export default function InitialFetch() {
    const [data, setData] = useState<any>("");
    const [loading, setLoading] = useState(false);
    const [error, setError] = useState<string|null>(null);

    return <>
         {error && <Alert severity="error">{error}</Alert>}
<LoadingButton
    loading={loading} onClick={async (event) => {
        setError(null);
        setLoading(true);
    
        try {
          let response = await fetch("http://localhost:8080/setup", {
            credentials: "include",
            method: "POST",
            headers: {
              "x-csrf-protection": "tucant",
            },
            body: "",
          });
          
          let reader = response.body?.getReader()
          let value: ReadableStreamReadResult<Uint8Array> | undefined;
          while (!(value = await reader?.read())?.done) {
            setData((data: string) => new TextDecoder().decode(value?.value));
          }

        } catch (error) {
          setError(String(error));
        } finally {
          setLoading(false);
        }
     }}>Initial sync</LoadingButton>
     {data}
     </>
}