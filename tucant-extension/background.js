import "./fix-session-id-in-url.js"

console.log("background script")

const EXTENSION_PAGE = chrome.runtime.getURL('/');

chrome.contextMenus.onClicked.addListener((info, tab) => {
    chrome.notifications.create({
        type: "basic",
        iconUrl: chrome.runtime.getURL("/icon-512.png"),
        title: "Unfortunately this feature is not implemented yet",
        message: "We welcome any contribution",
    });
})

const EXT_PAGE_INDEX_HTML = chrome.runtime.getURL('/dist/index.html');

chrome.runtime.onInstalled.addListener(async () => {
    console.log("on installed")

    await chrome.declarativeNetRequest.updateDynamicRules({
        removeRuleIds: [4100], // TODO check that rules have no dupes
        addRules: [{
            id: 4100,
            action: {
                type: /** @type {chrome.declarativeNetRequest.RuleActionType} */ ('redirect'),
                redirect: {
                    // I think this needs to statically be an allowed url
                    regexSubstitution: `${EXT_PAGE_INDEX_HTML}#\\1`,
                },
            },
            condition: {
                isUrlFilterCaseSensitive: true,
                resourceTypes: [
                    /** @type {chrome.declarativeNetRequest.ResourceType} */ ("main_frame")
                ],
                regexFilter: `^https://tucant\\.selfmade4u\\.de/#(.*)$`
            }
        }],
    });

    let tabs = await chrome.tabs.query({
        url: `https://tucant.selfmade4u.de/*`
    })

    await Promise.all(tabs.map(tab => {
        if (tab.id) {
            chrome.tabs.reload(tab.id)
        }
    }))

    await chrome.contextMenus.removeAll();

    chrome.contextMenus.create({
        id: "open-in-tucan",
        title: "Open in TUCaN",
        contexts: ["link"],
        targetUrlPatterns: [`${EXTENSION_PAGE}*`]
    }, () => {
        console.log(chrome.runtime.lastError)
    })

    chrome.contextMenus.create({
        id: "open-in-tucant",
        title: "Open in TUCaN't",
        contexts: ["link"],
        targetUrlPatterns: ["https://www.tucan.tu-darmstadt.de/*"]
    }, () => {
        console.log(chrome.runtime.lastError)
    })

    chrome.contextMenus.create({
        id: "open-in-tucan-page",
        title: "Open in TUCaN",
        contexts: ["page"],
        documentUrlPatterns: [`${EXTENSION_PAGE}*`]
    }, () => {
        console.log(chrome.runtime.lastError)
    })

    chrome.contextMenus.create({
        id: "open-in-tucant-page",
        title: "Open in TUCaN't",
        contexts: ["page"],
        documentUrlPatterns: ["https://www.tucan.tu-darmstadt.de/*"]
    }, () => {
        console.log(chrome.runtime.lastError)
    })
});

chrome.storage.sync.onChanged.addListener(async (changes) => {
    for (let [key, { oldValue, newValue }] of Object.entries(changes)) {
        if (key === "mobileDesign") {
            if (newValue) {
                enableMobileDesign()
            } else {
                disableMobileDesign()
            }
        }
        if (key === "customUi") {
            if (newValue) {
                enableCustomUi()
            } else {
                disableCustomUi()
            }
        }
    }
});

const EXT_PAGE = chrome.runtime.getURL('/dist/index.html');
/** @type {chrome.declarativeNetRequest.Rule[]} */
const customUiRules = [{
    id: 200,
    priority: 3,
    action: {
        type: /** @type {chrome.declarativeNetRequest.RuleActionType} */ ('redirect'),
        redirect: { regexSubstitution: EXT_PAGE + '#/registration/' },
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

function enableCustomUi() {
    chrome.declarativeNetRequest.updateDynamicRules({
        removeRuleIds: customUiRules.map(r => r.id),
        addRules: customUiRules,
    })
}

function disableCustomUi() {
    chrome.declarativeNetRequest.updateDynamicRules({
        removeRuleIds: customUiRules.map(r => r.id)
    })
}

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

// ensure its on when still loading the settings
enableCustomUi()
enableMobileDesign()

chrome.storage.sync.get(
    { mobileDesign: false, customUi: true },
).then(({ mobileDesign, customUi }) => {
    if (mobileDesign) {
        enableMobileDesign()
    } else {
        disableMobileDesign()
    }
    if (customUi) {
        enableCustomUi()
    } else {
        disableCustomUi()
    }
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

export { }