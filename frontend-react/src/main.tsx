import CssBaseline from "@mui/material/CssBaseline";
import React from "react";
import ReactDOM from "react-dom/client";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import Login from "./routes/Login";
import MiniDrawer from "./MiniDrawer";
import Welcome from "./routes/Welcome";
import Module from "./routes/Module";
import SearchModules from "./routes/SearchModules";
import SearchCourses from "./routes/SearchCourses";
import Course from "./routes/Course";
import Modules from "./routes/Modules";
import Credits from "./routes/Credits";

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
          <Route path="modules/*" element={<Modules />} />
          <Route path="module/*" element={<Module />} />
          <Route path="course/*" element={<Course />} />
          <Route path="credits" element={<Credits />} />
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
