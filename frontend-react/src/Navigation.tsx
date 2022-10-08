// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later
import { Suspense, useTransition } from "react";
import {
  NavLink,
  NavLinkProps,
  Outlet,
  useLinkClickHandler,
  useNavigate,
} from "react-router-dom";
import { useAppSelector } from "./redux/hooks";

export function Link(props: NavLinkProps) {
  const [isPending, startTransition] = useTransition();

  const internalOnClick = useLinkClickHandler(props.to, {
    replace: props.replace,
    state: props.state,
    target: props.target,
    preventScrollReset: props.preventScrollReset,
    relative: props.relative,
  });

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

export default function Navigation() {
  const isLoggedIn = useAppSelector((state) => state.user.loggedIn);

  return (
    <>
      <nav className="navbar navbar-expand-lg bg-light">
        <div className="container-fluid">
          <Link className="navbar-brand" to="/">
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
              {/*
       
          <RouterLink
            to="/my-modules/"
            text="Meine Module"
            icon={<ViewModuleIcon />}
          ></RouterLink>
          <RouterLink
            to="/search-modules"
            text="Modulsuche"
            icon={<SearchOutlined />}
          ></RouterLink>
          <RouterLink
            to="/search-courses"
            text="Veranstaltungssuche"
            icon={<SearchOutlined />}
          ></RouterLink>
         
          
          */}
            </ul>
          </div>
        </div>
      </nav>
      <Suspense>
        <Outlet />
      </Suspense>
    </>
  );
}
