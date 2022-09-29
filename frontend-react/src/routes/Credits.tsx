import { Typography } from "@mui/material";

export default function Credits() {
  return (
    <>
      <Typography variant="h1" align="center">
        Credits
      </Typography>
      <Typography variant="h2" align="center">
        Software
      </Typography>
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
      <Typography variant="h2" align="center">
        Logo
      </Typography>
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
          ></img>
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
    </>
  );
}
