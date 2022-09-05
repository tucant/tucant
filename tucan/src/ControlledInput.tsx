import TextField from "@mui/material/TextField";
import { useState } from "react";



export default function ControlledInput() {
    const [value, setValue] = useState("");

    return (
        <TextField
              value={value}
              onChange={(event) => setValue(event.target.value)} 
              margin="normal"
              required
              fullWidth
              id="username"
              label="TU-ID"
              name="username"
              autoComplete="username"
              autoFocus
            />
    )
}