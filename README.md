# Usage

Install Tampermonkey.
Add a Tampermonkey script with the following content:
```javascript
// ==UserScript==
// @name         tucant
// @namespace    https://www.tucan.tu-darmstadt.de
// @version      2024-10-24
// @description  A nicer, faster and more featureful frontend to TUCaN
// @author       Moritz Hedtke <Moritz.Hedtke@t-online.de>
// @match        https://www.tucan.tu-darmstadt.de/*
// @run-at       document-start
// @grant        GM_addElement
// ==/UserScript==

GM_addElement('script', {
    src: 'https://tucant.github.io/tucant/tucan-injector.js',
    type: 'text/javascript'
});
```

# Development

```
nix develop
cd tucan-injector
bacon

cd tucan-injector/dist
python -m http.server
```

Install Tampermonkey.
Add a Tampermonkey script with the following content:

```javascript
// ==UserScript==
// @name         New Userscript
// @namespace    https://www.tucan.tu-darmstadt.de
// @version      2024-10-18
// @description  try to take over the world!
// @author       You
// @match        https://www.tucan.tu-darmstadt.de/*
// @run-at       document-start
// @grant        GM_addElement
// ==/UserScript==

GM_addElement('script', {
    src: 'http://localhost:8000/tucan-injector.js',
    type: 'text/javascript'
});
```
