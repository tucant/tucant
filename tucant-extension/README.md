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