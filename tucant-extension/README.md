# tucant-extension

The Campus-Management System of TU Darmstadt called TUCaN lacks quality and usability. Therefore this extension adds some quality of life improvements.

* TUCaN sometimes chains redirects. In some known places the intermediate redirect is skipped by the extension to speed up the navigation. Furthermore, there are some pages that wait for 500 milliseconds before redirecting. These pages are also skipped at some known places.
* The URL contains your session ID. Therefore sharing URLs with others does not work. The extension automatically changes the session ID in the url to your session ID so sharing URLs works for users of the extension.

The screenshot shows one exemplary redirect page which is now not shown any more leading to a faster navigation.

The source code of this extension is available at https://github.com/tucant/tucant/tree/main/tucant-extension

```
 /*"web_accessible_resources": [
        {
            "resources": [
                "dist/index.html"
            ],
            "matches": [
                "https://www.tucan.tu-darmstadt.de/*"
            ]
        }
    ],
     "content_security_policy": {
        "extension_pages": "script-src 'self' 'wasm-unsafe-eval'; object-src 'self';"
    },*/

zip -1 tucant.zip rules.json manifest.json content-script.js icon.png


web-ext run -t firefox-android --adb-device XXX --firefox-apk org.mozilla.firefox

/
->
/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=STARTPAGE_DISPATCH&ARGUMENTS=-N000000000000001
-> 
/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N000000000000001,-N000344,-Awelcome


/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=STARTPAGE_DISPATCH&ARGUMENTS=-N$ID,-N000019,-N000000000000000
->
/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MLSSTART&ARGUMENTS=-N$ID,-N000019,
```