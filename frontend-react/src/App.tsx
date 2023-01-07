// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import React, { useTransition } from "react";
import ReactDOM from "react-dom/client";
import { BrowserRouter, Routes, Route } from "react-router-dom";
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
import { NavigationContext } from "./NavigationContext";
import MyCourses from "./routes/MyCourses";
import { SearchModulesOpenSearch } from "./routes/SearchModulesOpenSearch";
import MyExams from "./routes/MyExams";
import Exam from "./routes/Exam";
import CourseGroup from "./routes/CourseGroup";

function App() {
  const [isLoading, startTransition] = useTransition();

  return (
    <React.StrictMode>
      <SWRConfig value={{ suspense: true }}>
        <NavigationContext.Provider value={startTransition}>
          <BrowserRouter>
            <Routes>
              <Route path="/" element={<Navigation isLoading={isLoading} />}>
                <Route path="modules/" element={<Modules />} />
                <Route path="modules/:id" element={<Modules />} />
                <Route path="login" element={<Login />} />
                <Route path="my-modules" element={<MyModules />} />
                <Route path="my-courses" element={<MyCourses />} />
                <Route path="search-courses" element={<SearchCourses />} />
                <Route path="logout" element={<Logout />} />
                <Route path="search-modules" element={<SearchModules />} />
                <Route path="my-exams" element={<MyExams />} />
                <Route
                  path="search-modules-opensearch"
                  element={<SearchModulesOpenSearch />}
                />
                <Route path="module/:id" element={<Module />} />
                <Route path="course/:id" element={<Course />} />
                <Route path="course-group/:id" element={<CourseGroup />} />
                <Route path="exam/:id" element={<Exam />} />
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
        </NavigationContext.Provider>
      </SWRConfig>
    </React.StrictMode>
  );
}

const root = ReactDOM.createRoot(
  document.getElementById("root") as HTMLElement,
);
root.render(<App />);
