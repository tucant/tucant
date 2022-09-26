import CssBaseline from "@mui/material/CssBaseline";
import React from "react";
import ReactDOM from "react-dom/client";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import Login from "./routes/Login";
import MiniDrawer from "./MiniDrawer";
import Welcome from "./routes/Welcome";
import Modules from "./routes/Modules";
import Module from "./routes/Module";
import ModuleOrModules from "./routes/ModuleOrModules";
import SearchModules from "./routes/SearchModules";
import SearchCourses from "./routes/SearchCourses";

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
          <Route path="search-modules" element={<SearchModules />} />
          <Route path="search-courses" element={<SearchCourses />} />
          <Route path="modules/*" element={<ModuleOrModules />} />
          <Route index element={<Welcome />} />
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
