// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

import Navigation from "./Navigation";
import { NavigationContext } from "./NavigationContext";
import Course from "./routes/Course";
import CourseGroup from "./routes/CourseGroup";
import Courses from "./routes/Courses";
import Credits from "./routes/Credits";
import Exam from "./routes/Exam";
import Login from "./routes/Login";
import Logout from "./routes/Logout";
import Module from "./routes/Module";
import Modules from "./routes/Modules";
import MyCourses from "./routes/MyCourses";
import MyExams from "./routes/MyExams";
import MyModules from "./routes/MyModules";
import { SearchCourses } from "./routes/SearchCourses";
import { SearchModules } from "./routes/SearchModules";
import { SearchModulesOpenSearch } from "./routes/SearchModulesOpenSearch";
import Welcome from "./routes/Welcome";
import "./scss/styles.scss";
import React, { useTransition } from "react";
import ReactDOM from "react-dom/client";
import { BrowserRouter, Route, Routes } from "react-router-dom";
import { SWRConfig } from "swr";

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
                <Route path="vv/" element={<Courses />} />
                <Route path="vv/:id" element={<Courses />} />
                <Route path="login" element={<Login />} />
                <Route path="my-modules" element={<MyModules />} />
                <Route path="my-courses" element={<MyCourses />} />
                <Route path="search-courses" element={<SearchCourses />} />
                <Route path="logout" element={<Logout />} />
                <Route path="search-modules" element={<SearchModules />} />
                <Route path="my-exams/:semester" element={<MyExams />} />
                <Route
                  path="search-modules-opensearch"
                  element={<SearchModulesOpenSearch />}
                />
                <Route path="module/:id" element={<Module />} />
                <Route path="course/:id" element={<Course />} />
                <Route path="course-group/:id" element={<CourseGroup />} />
                <Route path="exam/:id" element={<Exam />} />
                <Route path="credits" element={<Credits />} />
                <Route
                  path="not-logged-into-tucan"
                  element={
                    "You are not logged into TUCaN! Use this login hack after you logged in."
                  }
                />
                <Route index={true} element={<Welcome />} />
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
