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

export class ErrorBoundary extends React.Component<any> {
  state = { hasError: false, error: null };
  static getDerivedStateFromError(error: any) {
    console.error(error);
    return {
      hasError: true,
      error,
    };
  }
  render() {
    if (this.state.hasError) {
      return (
        <div className="alert alert-success" role="alert">
          <h4 className="alert-heading">{this.props.fallback}</h4>
          <p>Die Fehlermeldung lautet: ${String(this.state.error)}</p>
          <hr />
          <p className="mb-0">
            Eventuell hilft die Fehlermeldung einem Entwickler.
          </p>
        </div>
      );
    }
    return this.props.children;
  }
}

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

                {/*
              <Route path="logout" element={<Logout />} />
              <Route path="search-modules" element={<SearchModules />} />
              <Route path="search-courses" element={<SearchCourses />} />
              <Route path="module/:id" element={<Module />} />
              <Route path="course/:id" element={<Course />} />
              <Route path="my-modules" element={<MyModules />} />
              <Route path="credits" element={<Credits />} />
            */}
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
