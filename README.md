```
nix develop
cd tucan-injector
bacon

cd tucan-injector/dist
python -m http.server
```

Install Tampermonkey.
Add a Tampermonkey script with the following content:

```
// ==UserScript==
// @name         New Userscript
// @namespace    https://www.tucan.tu-darmstadt.de
// @version      2024-10-18
// @description  try to take over the world!
// @author       You
// @match        https://www.tucan.tu-darmstadt.de/*
// @grant        none
// @require      http://localhost:8000/tucan-injector.js
// ==/UserScript==
```
