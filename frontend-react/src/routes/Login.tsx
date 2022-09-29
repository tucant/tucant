import * as React from "react";
import Avatar from "@mui/material/Avatar";
import CssBaseline from "@mui/material/CssBaseline";
import TextField from "@mui/material/TextField";
import Box from "@mui/material/Box";
import LockOutlinedIcon from "@mui/icons-material/LockOutlined";
import Typography from "@mui/material/Typography";
import Container from "@mui/material/Container";
import { createTheme, ThemeProvider } from "@mui/material/styles";
import { useState } from "react";
import LoadingButton from "@mui/lab/LoadingButton";
import Alert from "@mui/material/Alert";
import { useNavigate } from "react-router-dom";
import { LoginResponseSchema } from "../validation-io-ts";
import { isLeft } from "fp-ts/lib/Either";
import { PathReporter } from "io-ts/lib/PathReporter";
import { keyof } from "io-ts";

const theme = createTheme();

export default function SignIn() {
  const navigate = useNavigate();

  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const [form, setForm] = useState({
    username: "",
    password: "",
  });

  const handleInputChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const target = event.target;
    const value = target.type === "checkbox" ? target.checked : target.value;

    const AllowedNames = keyof({
      username: null,
      password: null,
    });
    const name = AllowedNames.decode(target.name);
    if (isLeft(name)) {
      throw new Error(
        `Internal Error: Invalid data format in response ${PathReporter.report(
          name
        ).join("\n")}`
      );
    }
    setForm({
      ...form,
      [name.right]: value,
    });
  };

  const handleSubmit = (event: React.ChangeEvent<HTMLFormElement>) => {
    event.preventDefault();

    (async () => {
      setError(null);
      setLoading(true);

      const response = await fetch("http://localhost:8080/login", {
        credentials: "include",
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          "x-csrf-protection": "tucant",
        },
        body: JSON.stringify(form),
      });
      const actualData = LoginResponseSchema.decode(await response.json());
      if (isLeft(actualData)) {
        throw new Error(
          `Internal Error: Invalid data format in response ${PathReporter.report(
            actualData
          ).join("\n")}`
        );
      }
      if (actualData.right.success) {
        navigate("/");
      } else {
        setError(String("Falscher Nutzername oder falsches Passwort!"));
      }
    })()
      .catch((error) => setError(String(error)))
      .finally(() => setLoading(false));
  };

  return (
    <ThemeProvider theme={theme}>
      <Container component="main" maxWidth="xs">
        <CssBaseline />
        <Box
          sx={{
            marginTop: 8,
            display: "flex",
            flexDirection: "column",
            alignItems: "center",
          }}
        >
          <Avatar sx={{ m: 1, bgcolor: "secondary.main" }}>
            <LockOutlinedIcon />
          </Avatar>
          <Typography component="h1" variant="h5">
            Anmelden
          </Typography>
          <Box
            component="form"
            onSubmit={handleSubmit}
            noValidate
            sx={{ mt: 1 }}
          >
            {error && <Alert severity="error">{error}</Alert>}

            <TextField
              onChange={handleInputChange}
              value={form.username}
              margin="normal"
              required
              fullWidth
              id="username"
              label="TU-ID"
              name="username"
              autoComplete="username"
              autoFocus
            />
            <TextField
              onChange={handleInputChange}
              value={form.password}
              margin="normal"
              required
              fullWidth
              name="password"
              label="Passwort"
              type="password"
              id="password"
              autoComplete="current-password"
            />
            <LoadingButton
              loading={loading}
              type="submit"
              fullWidth
              variant="contained"
              sx={{ mt: 3, mb: 2 }}
            >
              Anmelden
            </LoadingButton>
          </Box>
        </Box>
      </Container>
    </ThemeProvider>
  );
}
