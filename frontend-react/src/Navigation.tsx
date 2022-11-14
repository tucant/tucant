// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later
import { ReactNode, Suspense, useContext } from "react";
import {
  NavLink,
  NavLinkProps,
  Outlet,
  useLinkClickHandler,
} from "react-router-dom";
import { ErrorBoundary } from "./components/ErrorBoundary";
import { NavigationContext } from "./NavigationContext";
import { useAppSelector } from "./redux/hooks";

export function Link({ ...props }: NavLinkProps & { children: ReactNode }) {
  const startTransition = useContext(NavigationContext);

  const internalOnClick = useLinkClickHandler(props.to, props);

  return (
    <NavLink
      onClick={(event) => {
        event.preventDefault();
        startTransition(() => {
          internalOnClick(event);
        });
      }}
      {...props}
    ></NavLink>
  );
}

export default function Navigation({ isLoading }: { isLoading: boolean }) {
  const isLoggedIn = useAppSelector((state) => state.user.loggedIn);

  return (
    <div className="vh-100 position-relative">
      <nav className="navbar navbar-expand-lg bg-light">
        <div className="container-fluid">
          <Link className="navbar-brand" to="/">
            <img
              src="/TUCaNt.svg"
              alt="Logo"
              width="30"
              height="24"
              className="me-1 d-inline-block align-text-top"
            />
            TUCaN&apos;t
          </Link>
          <button
            className="navbar-toggler"
            type="button"
            data-bs-toggle="collapse"
            data-bs-target="#navbarSupportedContent"
            aria-controls="navbarSupportedContent"
            aria-expanded="false"
            aria-label="Toggle navigation"
          >
            <span className="navbar-toggler-icon"></span>
          </button>
          <div className="collapse navbar-collapse" id="navbarSupportedContent">
            <ul className="navbar-nav me-auto mb-2 mb-lg-0">
              <li className="nav-item">
                <Link className="nav-link" to="/" end>
                  Startseite
                </Link>
              </li>
              <li className="nav-item">
                <Link className="nav-link" to="/modules/">
                  Module
                </Link>
              </li>
              <li className="nav-item">
                <Link className="nav-link" to="/my-modules/">
                  Meine Module
                </Link>
              </li>
              <li className="nav-item">
                <Link className="nav-link" to="/my-courses/">
                  Meine Veranstaltungen
                </Link>
              </li>
              <li className="nav-item">
                <Link className="nav-link" to="/search-modules">
                  Modulsuche
                </Link>
              </li>
              <li className="nav-item">
                <Link className="nav-link" to="/search-modules-opensearch">
                  Modulsuche (OpenSearch)
                </Link>
              </li>
              <li className="nav-item">
                <Link className="nav-link" to="/search-courses">
                  Veranstaltungssuche
                </Link>
              </li>
              <li className="nav-item">
                <Link className="nav-link" to="/credits">
                  Credits
                </Link>
              </li>
              {!isLoggedIn && (
                <li className="nav-item">
                  <Link className="nav-link" to="/login">
                    Login
                  </Link>
                </li>
              )}
              {isLoggedIn && (
                <li className="nav-item">
                  <Link className="nav-link" to="/logout">
                    Logout
                  </Link>
                </li>
              )}
            </ul>
          </div>
        </div>
      </nav>
      <ErrorBoundary
        fallback={
          "Etwas ist richtig schief gelaufen. Versuche mal die Seite neu zu laden."
        }
      >
        <Suspense
          fallback={
            <div
              style={{ zIndex: 10000 }}
              className="position-absolute top-50 start-50 translate-middle"
            >
              <div className="spinner-grow" role="status">
                <span className="visually-hidden">Loading...</span>
              </div>
            </div>
          }
        >
          <Outlet />
        </Suspense>
      </ErrorBoundary>
      <div
        style={{ zIndex: 10000 }}
        className="position-absolute top-50 start-50 translate-middle"
      >
        {isLoading && (
          <div className="spinner-grow" role="status">
            <span className="visually-hidden">Loading...</span>
          </div>
        )}
      </div>
    </div>
  );
}
