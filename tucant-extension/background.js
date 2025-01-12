console.log("background script")

chrome.cookies.onChanged.addListener((changeInfo) => {
    if (changeInfo.cookie.name === "cnsc" && changeInfo.removed) {
        chrome.cookies.remove({
            url: "https://www.tucan.tu-darmstadt.de/scripts/",
            name: "id",
        })
    } else if (changeInfo.cookie.name === "id") {
        if (changeInfo.removed && changeInfo.cause !== "overwrite") {
            chrome.declarativeNetRequest.updateDynamicRules({
                removeRuleIds: fixupSessionIdInUrl("").map(r => r.id),
                addRules: [],
            });
            chrome.action.setBadgeText({ text: "" })
        } else {
            chrome.action.setBadgeText({ text: "L" })
            chrome.action.setBadgeBackgroundColor(
                { color: 'green' }
            )
            chrome.action.setBadgeTextColor({ color: "white" });
            chrome.declarativeNetRequest.updateDynamicRules({
                removeRuleIds: fixupSessionIdInUrl(changeInfo.cookie.value).map(r => r.id),
                addRules: fixupSessionIdInUrl(changeInfo.cookie.value),
            });
        }
    }
});

const fixupSessionIdInUrl = (/** @type {string} */ sessionId) => [{
    // redirect any session id to the currently valid session id
    id: 100,
    action: {
        type: /** @type {chrome.declarativeNetRequest.RuleActionType} */ ('redirect'),
        redirect: {
            regexSubstitution: `https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=\\1&ARGUMENTS=-N${sessionId},\\2`,
        },
    },
    condition: {
        isUrlFilterCaseSensitive: true,
        resourceTypes: [
            /** @type {chrome.declarativeNetRequest.ResourceType} */ ("main_frame")
        ],
        regexFilter: `^https://www\\.tucan\\.tu-darmstadt\\.de/scripts/mgrqispi\\.dll\\?APPNAME=CampusNet&PRGNAME=([A-Z]+)&ARGUMENTS=-N\\d+,(.+)$`
    }
}, {
    // but don't create an infinite loop
    id: 101,
    priority: 2,
    action: {
        type: /** @type {chrome.declarativeNetRequest.RuleActionType} */ ('allow')
    },
    condition: {
        isUrlFilterCaseSensitive: true,
        resourceTypes: [
            /** @type {chrome.declarativeNetRequest.ResourceType} */ ("main_frame")
        ],
        regexFilter: `^https://www\\.tucan\\.tu-darmstadt\\.de/scripts/mgrqispi\\.dll\\?APPNAME=CampusNet&PRGNAME=([A-Z]+)&ARGUMENTS=-N${sessionId},(.+)$`
    }
}, {
    // and don't redirect explicitly unauthenticated urls
    id: 102,
    priority: 2,
    action: {
        type: /** @type {chrome.declarativeNetRequest.RuleActionType} */ ('allow')
    },
    condition: {
        isUrlFilterCaseSensitive: true,
        resourceTypes: [
            /** @type {chrome.declarativeNetRequest.ResourceType} */ ("main_frame")
        ],
        regexFilter: `^https://www\\.tucan\\.tu-darmstadt\\.de/scripts/mgrqispi\\.dll\\?APPNAME=CampusNet&PRGNAME=([A-Z]+)&ARGUMENTS=-N000000000000001,(.+)$`
    }
}];

chrome.webRequest.onBeforeRequest.addListener((details) => {
    if (details.url === "https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll" && details.method === "POST") {
        console.log("login attempt")
        chrome.cookies.remove({
            url: "https://www.tucan.tu-darmstadt.de/scripts/",
            name: "id",
        })
    }
}, { urls: ["https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll"] })

chrome.webRequest.onHeadersReceived.addListener((details) => {
    if (details.url === "https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll" && details.method === "POST") {
        const refreshHeader = details.responseHeaders?.filter(v => v.name === "REFRESH").map(v => v.value).find(v => true) ?? "";
        const match = new RegExp("^0; URL=/scripts/mgrqispi\\.dll\\?APPNAME=CampusNet&PRGNAME=STARTPAGE_DISPATCH&ARGUMENTS=-N(\\d+),-N\\d+,-N000000000000000$", "g").exec(refreshHeader);
        if (match !== null) {
            const sessionId = match[1]

            chrome.cookies.set({
                url: "https://www.tucan.tu-darmstadt.de/scripts/",
                name: "id",
                value: sessionId,
                secure: true
            })
        }
    }

    const logoutMatch = new RegExp("^https://www\\.tucan\\.tu-darmstadt\\.de/scripts/mgrqispi\\.dll\\?APPNAME=CampusNet&PRGNAME=LOGOUT&.*$", "g").exec(details.url);
    if (logoutMatch !== null) {
        chrome.cookies.remove({
            url: "https://www.tucan.tu-darmstadt.de/scripts/",
            name: "id",
        })
    }
}, { urls: ["https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll", "https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=LOGOUT&*"] }, ["responseHeaders"]);

chrome.runtime.onInstalled.addListener(() => {
    console.log("on installed")

    const EXT_PAGE = chrome.runtime.getURL('/dist/index.html');
    /** @type {chrome.declarativeNetRequest.Rule[]} */
    const RULES = [{
        id: 200,
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
    }, {
        id: 201,
        priority: 3,
        action: {
            type: /** @type {chrome.declarativeNetRequest.RuleActionType} */ ('redirect'),
            redirect: { regexSubstitution: EXT_PAGE + '#/' },
        },
        "condition": {
            "isUrlFilterCaseSensitive": true,
            "resourceTypes": [
                /** @type {chrome.declarativeNetRequest.ResourceType} */ ("main_frame")
            ],
            "regexFilter": "^https://www\\.tucan\\.tu-darmstadt\\.de/$"
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
});


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

// ensure state is set on extension enable
chrome.cookies.get({
    url: "https://www.tucan.tu-darmstadt.de/scripts/",
    name: "id",
}).then(idCookie => {
    if (idCookie) {
        chrome.action.setBadgeText({ text: "L" })
        chrome.action.setBadgeBackgroundColor(
            { color: 'green' }
        )
        chrome.action.setBadgeTextColor({ color: "white" });
        chrome.declarativeNetRequest.updateDynamicRules({
            removeRuleIds: fixupSessionIdInUrl(idCookie.value).map(r => r.id),
            addRules: fixupSessionIdInUrl(idCookie.value),
        });
    } else {
        chrome.declarativeNetRequest.updateDynamicRules({
            removeRuleIds: fixupSessionIdInUrl("").map(r => r.id),
            addRules: [],
        });
        chrome.action.setBadgeText({ text: "" })
    }
})

export { }