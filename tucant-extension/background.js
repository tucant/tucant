console.log("background script")

// maybe don't extract the cookie using the cookie api at all but instead use a content script to extract the info from the tucan page. then the id and cookie can't get out of sync.
// and we can get the current value just with a reload of a tucan page and not a new login.
// but also won't get auto-deleted. though we could store a hash of the cookie or so to compare if its still up to date

chrome.runtime.onInstalled.addListener(() => {
    console.log("on installed")

    const EXT_PAGE = chrome.runtime.getURL('/dist/index.html');
    /** @type {chrome.declarativeNetRequest.Rule[]} */
    const RULES = [{
        id: 1337,
        priority: 3,
        action: {
            type: /** @type {chrome.declarativeNetRequest.RuleActionType} */ ('redirect'),
            redirect: { regexSubstitution: EXT_PAGE + '#/registration/,-N000311,-A' },
        },
        "condition": {
            "isUrlFilterCaseSensitive": true,
            "resourceTypes": [
                /** @type {chrome.declarativeNetRequest.ResourceType} */ ("main_frame")
            ],
            "regexFilter": "^https://www\\.tucan\\.tu-darmstadt\\.de/scripts/mgrqispi\\.dll\\?APPNAME=CampusNet&PRGNAME=REGISTRATION&ARGUMENTS=-N(\\d+),-N000311,-A$"
        }
    }];
    chrome.declarativeNetRequest.updateDynamicRules({
        removeRuleIds: RULES.map(r => r.id),
        addRules: RULES,
    }).then(() => {
        console.log("registered")
    }).catch(error => {
        console.error(error)
    });
});

chrome.omnibox.onInputStarted.addListener(function () {
    chrome.omnibox.setDefaultSuggestion({
        description: "TUCaN't"
    });
});

chrome.omnibox.onInputChanged.addListener(event => {
    chrome.omnibox.setDefaultSuggestion({
        description: "TUCaN't"
    });

    /** @type {chrome.omnibox.SuggestResult[]} */
    let results = [{
        content: "https://www.tucan.tu-darmstadt.de",
        description: "TUCaN't"
    }]
    return results
})

chrome.omnibox.onInputEntered.addListener((event) => {
    chrome.tabs.update({ url: "https://www.tucan.tu-darmstadt.de" })
})

chrome.storage.sync.onChanged.addListener((changes) => {
    for (let [key, { oldValue, newValue }] of Object.entries(changes)) {
        console.log(
            `Storage key "${key}" changed.`,
            `Old value was "${oldValue}", new value is "${newValue}".`
        );
        if (key === "mobileDesign") {
            if (newValue) {
                enableMobileDesign()
            } else {
                disableMobileDesign()
            }
        }
    }
});

function enableMobileDesign() {
    chrome.scripting.registerContentScripts(
        [{
            id: "mobile",
            "matches": [
                "https://www.tucan.tu-darmstadt.de/*"
            ],
            "css": [
                "mobile.css"
            ],
            "js": [
                "mobile.js"
            ],
            "runAt": "document_end"
        }]
    )
}

function disableMobileDesign() {
    chrome.scripting.unregisterContentScripts({
        ids: ["mobile"]
    })
}

chrome.webRequest.onBeforeRequest.addListener((details) => {
    console.log(details)
    if (details.url === "https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll" && details.method === "POST") {
        console.log("login attempt")
        chrome.declarativeNetRequest.updateDynamicRules({
            removeRuleIds: [1338, 1339, 1340],
            addRules: [],
        });
        chrome.action.setBadgeText({ text: "" })
    }
}, { urls: ["https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll"] })

chrome.webRequest.onHeadersReceived.addListener((details) => {
    console.log(details)
    if (details.url === "https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll" && details.method === "POST") {
        console.log("login attempt")
        chrome.declarativeNetRequest.updateDynamicRules({
            removeRuleIds: [1338, 1339, 1340],
            addRules: [],
        });
        chrome.action.setBadgeText({ text: "" })

        const refreshHeader = details.responseHeaders?.filter(v => v.name === "REFRESH").map(v => v.value).find(v => true) ?? "";
        console.log(refreshHeader)
        const match = new RegExp("^0; URL=/scripts/mgrqispi\\.dll\\?APPNAME=CampusNet&PRGNAME=STARTPAGE_DISPATCH&ARGUMENTS=-N(\\d+),-N000019,-N000000000000000$", "g").exec(refreshHeader);
        if (match !== null) {
            const sessionId = match[1]
            console.log(`logged in with session id ${sessionId}`);
            chrome.action.setBadgeText({ text: "L" })
            chrome.action.setBadgeBackgroundColor(
                { color: 'green' }
            )
            chrome.action.setBadgeTextColor({ color: "white" });

            /** @type {chrome.declarativeNetRequest.Rule[]} */
            const RULES = [{
                id: 1338,
                action: {
                    type: /** @type {chrome.declarativeNetRequest.RuleActionType} */ ('redirect'),
                    redirect: {
                        regexSubstitution: `https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=\\1&ARGUMENTS=-N${sessionId},\\2`,
                    },
                },
                "condition": {
                    "isUrlFilterCaseSensitive": true,
                    "resourceTypes": [
                        /** @type {chrome.declarativeNetRequest.ResourceType} */ ("main_frame")
                    ],
                    "regexFilter": `^https://www\\.tucan\\.tu-darmstadt\\.de/scripts/mgrqispi\\.dll\\?APPNAME=CampusNet&PRGNAME=([A-Z]+)&ARGUMENTS=-N\\d+,(.+)$`
                }
            }, {
                id: 1339,
                priority: 2,
                action: {
                    type: /** @type {chrome.declarativeNetRequest.RuleActionType} */ ('allow')
                },
                "condition": {
                    "isUrlFilterCaseSensitive": true,
                    "resourceTypes": [
                        /** @type {chrome.declarativeNetRequest.ResourceType} */ ("main_frame")
                    ],
                    "regexFilter": `^https://www\\.tucan\\.tu-darmstadt\\.de/scripts/mgrqispi\\.dll\\?APPNAME=CampusNet&PRGNAME=([A-Z]+)&ARGUMENTS=-N${sessionId},(.+)$`
                }
            }, {
                id: 1340,
                priority: 2,
                action: {
                    type: /** @type {chrome.declarativeNetRequest.RuleActionType} */ ('allow')
                },
                "condition": {
                    "isUrlFilterCaseSensitive": true,
                    "resourceTypes": [
                        /** @type {chrome.declarativeNetRequest.ResourceType} */ ("main_frame")
                    ],
                    "regexFilter": `^https://www\\.tucan\\.tu-darmstadt\\.de/scripts/mgrqispi\\.dll\\?APPNAME=CampusNet&PRGNAME=([A-Z]+)&ARGUMENTS=-N000000000000001,(.+)$`
                }
            }];
            chrome.declarativeNetRequest.updateDynamicRules({
                removeRuleIds: RULES.map(r => r.id),
                addRules: RULES,
            });

            chrome.storage.local.set(
                { sessionId },
            );
        }
    }

    console.log(details.url)
    const logoutMatch = new RegExp("^https://www\\.tucan\\.tu-darmstadt\\.de/scripts/mgrqispi\\.dll\\?APPNAME=CampusNet&PRGNAME=LOGOUT&.*$", "g").exec(details.url);
    if (logoutMatch !== null) {
        console.log(`logged out`);
        chrome.action.setBadgeText({ text: "" })

        chrome.storage.local.remove("sessionId")
    }
}, { urls: ["https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll", "https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=LOGOUT&*"] }, ["responseHeaders"]);

chrome.contextMenus.create({
    id: "open-in-tucan",
    title: "Open in TUCaN",
    targetUrlPatterns: ["https://www.tucan.tu-darmstadt.de/*"]
})

chrome.contextMenus.create({
    id: "open-in-tucant",
    title: "Open in TUCaN't",
    targetUrlPatterns: ["https://www.tucan.tu-darmstadt.de/*"]
})