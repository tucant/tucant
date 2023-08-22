// SPDX-FileCopyrightText: The tucant Contributors
//
// SPDX-License-Identifier: AGPL-3.0-or-later

export default function Credits() {
  return (
    <main className="container">
      <h1 className="text-center">Credits</h1>
      <h2 className="text-center">Software</h2>
      <a
        target="_blank"
        href="https://github.com/mohe2015/tucant"
        rel="noreferrer"
      >
        tucant
      </a>{" "}
      von den{" "}
      <a
        target="_blank"
        rel="cc:attributionURL noreferrer"
        href="https://github.com/mohe2015/tucant/graphs/contributors"
      >
        tucant-Mitwirkenden
      </a>{" "}
      ist{" "}
      <a
        target="_blank"
        rel="license noreferrer"
        href="https://www.gnu.org/licenses/agpl-3.0.html"
      >
        AGPL-3.0
      </a>{" "}
      lizensiert.
      <p>
        tucant - a nicer, faster and more featureful frontend to TUCaN
        <br />
        Copyright (C) The tucant Contributors
        <br />
        This program is free software: you can redistribute it and/or modify it
        under the terms of the GNU Affero General Public License as published by
        the Free Software Foundation, either version 3 of the License, or (at
        your option) any later version.
        <br />
        This program is distributed in the hope that it will be useful, but
        WITHOUT ANY WARRANTY; without even the implied warranty of
        MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero
        General Public License for more details.
        <br />
        You should have received a copy of the GNU Affero General Public License
        along with this program. If not, see{" "}
        <a href="https://www.gnu.org/licenses/">
          https://www.gnu.org/licenses/
        </a>
        .
      </p>
      <h2 className="text-center">Logo</h2>
      <p>
        <a
          target="_blank"
          rel="license noreferrer"
          href="http://creativecommons.org/licenses/by-nc-sa/3.0/de/"
        >
          <img
            style={{ borderWidth: 0 }}
            src="http://i.creativecommons.org/l/by-nc-sa/3.0/de/88x31.png"
            alt="Creative Commons Lizenzvertrag"
          />
        </a>
        <br />
        <span>Das „TUCaN&apos;t“-Logo</span> von{" "}
        <a
          target="_blank"
          rel="cc:attributionURL noreferrer"
          href="http://daswesentliche.d120.de/"
        >
          Benedikt Bicker
        </a>{" "}
        steht unter einer{" "}
        <a
          target="_blank"
          rel="license noreferrer"
          href="http://creativecommons.org/licenses/by-nc-sa/3.0/de/"
        >
          Creative Commons Namensnennung-Nicht-kommerziell-Weitergabe unter
          gleichen Bedingungen 3.0 Deutschland Lizenz
        </a>
        .
      </p>
    </main>
  );
}
