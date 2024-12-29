const EXT_PAGE = chrome.runtime.getURL('/dist/index.html');
/** @type {chrome.declarativeNetRequest.Rule[]} */
const RULES = [{
    id: 1337,
    action: {
        type: /** @type {chrome.declarativeNetRequest.RuleActionType} */ ('redirect'),
        redirect: { regexSubstitution: EXT_PAGE + '#/registration/abc' },
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
});

// runtime.openOptionsPage()
// https://stackoverflow.com/questions/70640859/manifest-v3-pageaction-show

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
        }


    }

    console.log(details.url)
    const logoutMatch = new RegExp("^https://www\\.tucan\\.tu-darmstadt\\.de/scripts/mgrqispi\\.dll\\?APPNAME=CampusNet&PRGNAME=LOGOUT&.*$", "g").exec(details.url);
    if (logoutMatch !== null) {
        console.log(`logged out`);
        chrome.action.setBadgeText({ text: "" })
    }
}, { urls: ["https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll", "https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=LOGOUT&*"] }, ["responseHeaders"]);
