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