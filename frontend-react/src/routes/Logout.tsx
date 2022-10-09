// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import * as React from "react";
import { useNavigate } from "react-router-dom";
import { useAppDispatch } from "../redux/hooks";
import { setLoggedIn } from "../redux/user/userSlice";

export default function SignOut() {
  const navigate = useNavigate();

  const dispatch = useAppDispatch();

  // remove id cookie and redirect to home
  React.useEffect(() => {
    document.cookie = "id=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;";
    dispatch(setLoggedIn(false));
    navigate("/");
  }, []);

  return <main className="container">progress</main>;
}
