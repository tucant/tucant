# tucant-extension

The Campus-Management System of TU Darmstadt called TUCaN lacks quality and usability. Therefore this extension adds some quality of life improvements.

* TUCaN sometimes chains redirects. In some known places the intermediate redirect is skipped by the extension to speed up the navigation. Furthermore, there are some pages that wait for 500 milliseconds before redirecting. These pages are also skipped at some known places.
* The URL contains your session ID. Therefore sharing URLs with others does not work. The extension automatically changes the session ID in the url to your session ID so sharing URLs works for users of the extension.

The screenshot shows one exemplary redirect page which is now not shown any more leading to a faster navigation.

The source code of this extension is available at https://github.com/tucant/tucant/tree/main/tucant-extension

## Packaging

### Chromium

https://developer.chrome.com/docs/extensions/how-to/distribute/host-on-linux#packaging

Open Chromium -> chrome://extensions/ -> Pack extension -> Choose folder -> Pack. Store private key in a secure place

```bash
chromium --pack-extension=tucant-extension --pack-extension-key=/path/to/tucant-extension.pem
```

### Firefox

https://extensionworkshop.com/documentation/publish/distribute-sideloading/

ZIP the extension files.

For Developer Edition, Nightly, ESR: `about:config` `xpinstall.signatures.required` set to `false`.
Android only has Nightly available.

Otherwise upload to AMO as unlisted extension and pray that it gets signed quickly.

## Installation

## Development

```
web-ext run -t firefox-android --adb-device XXX --firefox-apk org.mozilla.firefox
```

```javascript
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
```
