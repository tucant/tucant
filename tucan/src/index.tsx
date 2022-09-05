import CssBaseline from "@mui/material/CssBaseline";
import React from "react";
import ReactDOM from "react-dom/client";
import reportWebVitals from "./reportWebVitals";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import Login from "./routes/Login";
import MiniDrawer from "./MiniDrawer";
import Welcome from "./routes/Welcome";
import Registration from "./routes/Registration";

const root = ReactDOM.createRoot(
  document.getElementById("root") as HTMLElement
);
root.render(
  <React.StrictMode>
    <CssBaseline enableColorScheme />
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<MiniDrawer />}>
          <Route path="login" element={<Login />} />
          <Route path="registration" element={<Registration />} />
          <Route
        index
        element={<Welcome />}
      />
       <Route
            path="*"
            element={
              <main style={{ padding: "1rem" }}>
                <p>Seite nicht gefunden!</p>
              </main>
            }
          />
        </Route>
      </Routes>
    </BrowserRouter>
  </React.StrictMode>
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
