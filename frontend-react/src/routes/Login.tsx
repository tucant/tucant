// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import * as React from "react";
import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { login } from "../api";
import { useAppDispatch, useAppSelector } from "../redux/hooks";
import { setLoggedIn } from "../redux/user/userSlice";

export default function SignIn() {
  const navigate = useNavigate();

  const isLoggedIn = useAppSelector((state) => state.user.loggedIn);

  // if cookie id is set, redirect to home
  React.useEffect(() => {
    if (isLoggedIn && false) navigate("/");
  }, [isLoggedIn]);

  const dispatch = useAppDispatch();

  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const [form, setForm] = useState({
    username: "",
    password: "",
  });

  const handleInputChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    const target = event.target;
    const value = target.type === "checkbox" ? target.checked : target.value;

    if (target.name != "username" && target.name != "password") {
      throw new Error("unexpected input name");
    }
    setForm({
      ...form,
      [target.name]: value,
    });
  };

  const handleSubmit = (event: React.ChangeEvent<HTMLFormElement>) => {
    event.preventDefault();

    (async () => {
      setError(null);
      setLoading(true);

      const response = await login(form);
      if (response.success) {
        dispatch(setLoggedIn(true));
        navigate("/");
      } else {
        setError(String("Falscher Nutzername oder falsches Passwort!"));
      }
    })()
      .catch((error) => setError(String(error)))
      .finally(() => setLoading(false));
  };

  return (
    <div className="container">
      <div className="row justify-content-md-center">
        <form className="col-3 mt-3" onSubmit={handleSubmit}>
          <h1 className="h3 mb-3 fw-normal">Anmelden</h1>

          <div className="form-floating">
            <input
              id="floatingInput"
              onChange={handleInputChange}
              value={form.username}
              name="username"
              required
              type="text"
              className="form-control"
              autoComplete="username"
            />
            <label htmlFor="floatingInput">TU-ID</label>
          </div>
          <div className="form-floating">
            <input
              id="floatingPassword"
              onChange={handleInputChange}
              name="password"
              value={form.password}
              type="password"
              required
              className="form-control"
              autoComplete="current-password"
            />
            <label htmlFor="floatingPassword">Passwort</label>
          </div>

          <button className="mt-3 w-100 btn btn-lg btn-primary" type="submit">
            Sign in
          </button>
        </form>
      </div>
    </div>
  );
}
