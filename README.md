```
nix develop
cd tucan-injector
bacon
```

You need to use Chromium to load the Tampermonkey script from a local file url.

Install https://chromewebstore.google.com/detail/tampermonkey/dhdgffkkebhmkfjojejmpbldmpobfkfo

In Chrome extension settings, enable "Allow access to file URLs".

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
// @require      file:///path/to/tucant/checkout/tucan-injector/dist/tucan-injector.js
// ==/UserScript==
```
