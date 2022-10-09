// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import React from "react";
import ReactDOM from "react-dom/client";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import { Provider } from "react-redux";
import { store } from "./redux/store";
import "./scss/styles.scss";
import Navigation from "./Navigation";
import Modules from "./routes/Modules";
import Login from "./routes/Login";
import Welcome from "./routes/Welcome";
import { SWRConfig } from "swr";
import MyModules from "./routes/MyModules";
import Logout from "./routes/Logout";
import { SearchModules } from "./routes/SearchModules";
import Module from "./routes/Module";
import Course from "./routes/Course";
import Credits from "./routes/Credits";
import { SearchCourses } from "./routes/SearchCourses";

function App() {
  return (
    <React.StrictMode>
      <SWRConfig value={{ suspense: true }}>
        <Provider store={store}>
          <BrowserRouter>
            <Routes>
              <Route path="/" element={<Navigation />}>
                <Route path="modules/" element={<Modules />} />
                <Route path="modules/:id" element={<Modules />} />
                <Route path="login" element={<Login />} />
                <Route path="my-modules" element={<MyModules />} />
                <Route path="search-courses" element={<SearchCourses />} />
                <Route path="logout" element={<Logout />} />
                <Route path="search-modules" element={<SearchModules />} />
                <Route path="module/:id" element={<Module />} />
                <Route path="course/:id" element={<Course />} />
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
        </Provider>
      </SWRConfig>
    </React.StrictMode>
  );
}

const root = ReactDOM.createRoot(
  document.getElementById("root") as HTMLElement
);
root.render(<App />);
