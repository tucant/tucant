import "./fix-session-id-in-url.js"

console.log("background script")

const EXTENSION_PAGE = chrome.runtime.getURL('/');
const EXT_PAGE_INDEX_HTML = chrome.runtime.getURL('/dist/index.html');

chrome.contextMenus.onClicked.addListener(async (info, tab) => {
    const id = await chrome.cookies.get({
        url: "https://www.tucan.tu-darmstadt.de/scripts/",
        name: "id",
    })

    let url = info.linkUrl || info.pageUrl

    let match = new RegExp("^https://www\\.tucan\\.tu-darmstadt\\.de/scripts/mgrqispi\\.dll\\?APPNAME=CampusNet&PRGNAME=MODULEDETAILS&ARGUMENTS=-N\\d+,-N\\d+,(.*)$", "g").exec(url)
    if (match) {
        chrome.tabs.create({
            url: `${EXT_PAGE_INDEX_HTML}#/module-details/${match[1]}`
        })
        return;
    }

    match = new RegExp("^https://www\\.tucan\\.tu-darmstadt\\.de/scripts/mgrqispi\\.dll\\?APPNAME=CampusNet&PRGNAME=COURSEDETAILS&ARGUMENTS=-N\\d+,-N\\d+,(.*)$", "g").exec(url)
    if (match) {
        chrome.tabs.create({
            url: `${EXT_PAGE_INDEX_HTML}#/course-details/${match[1]}`
        })
        return;
    }

    match = new RegExp("^https://www\\.tucan\\.tu-darmstadt\\.de/scripts/mgrqispi\\.dll\\?APPNAME=CampusNet&PRGNAME=REGISTRATION&ARGUMENTS=-N\\d+,-N\\d+,(.*)$", "g").exec(url)
    if (match) {
        chrome.tabs.create({
            url: `${EXT_PAGE_INDEX_HTML}#/registration/${match[1]}`
        })
        return;
    }

    match = new RegExp(`^${EXT_PAGE_INDEX_HTML}#/course-details/(.*)$`, "g").exec(url)
    if (match) {
        chrome.tabs.create({
            url: `https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=COURSEDETAILS&ARGUMENTS=-N${id},-N000274,${match[1]}`
        })
        return;
    }

    chrome.notifications.create({
        type: "basic",
        iconUrl: chrome.runtime.getURL("/icon-512.png"),
        title: "URL not supported",
        message: "Unfortunately this URL is not supported yet. We welcome any contribution",
    });
})

chrome.runtime.onInstalled.addListener(async () => {
    console.log("on installed")

    await chrome.declarativeNetRequest.updateDynamicRules({
        removeRuleIds: [4100], // TODO check that rules have no dupes
        addRules: [{
            id: 4100,
            priority: 10,
            condition: {
                isUrlFilterCaseSensitive: true,
                resourceTypes: [
/** @type {chrome.declarativeNetRequest.ResourceType} */ ("main_frame")
                ],
                regexFilter: `^https://tucant\\.selfmade4u\\.de/#(.*)$`
            },
            action: {
                type: /** @type {chrome.declarativeNetRequest.RuleActionType} */ ('redirect'),
                redirect: {
                    // I think this needs to statically be an allowed url
                    regexSubstitution: `${EXT_PAGE_INDEX_HTML}#\\1`,
                },
            },
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
    condition: {
        isUrlFilterCaseSensitive: true,
        resourceTypes: [
            /** @type {chrome.declarativeNetRequest.ResourceType} */ ("main_frame")
        ],
        regexFilter: "^https://www\\.tucan\\.tu-darmstadt\\.de/$"
    },
    action: {
        type: /** @type {chrome.declarativeNetRequest.RuleActionType} */ ('redirect'),
        redirect: { regexSubstitution: EXT_PAGE + '#/' },
    },
}, {
    // TODO normalize all the urls below, see AnmeldungRequest etc
    id: 201,
    priority: 3,
    condition: {
        isUrlFilterCaseSensitive: true,
        resourceTypes: [
            /** @type {chrome.declarativeNetRequest.ResourceType} */ ("main_frame")
        ],
        regexFilter: "^https://www\\.tucan\\.tu-darmstadt\\.de/scripts/mgrqispi\\.dll\\?APPNAME=CampusNet&PRGNAME=REGISTRATION&ARGUMENTS=-N\\d+,-N\\d+,(.*)$"
    },
    action: {
        type: /** @type {chrome.declarativeNetRequest.RuleActionType} */ ('redirect'),
        redirect: { regexSubstitution: EXT_PAGE + '#/registration/\\1' },
    },
}, {
    id: 202,
    priority: 3,
    condition: {
        isUrlFilterCaseSensitive: true,
        resourceTypes: [
            /** @type {chrome.declarativeNetRequest.ResourceType} */ ("main_frame")
        ],
        regexFilter: "^https://www\\.tucan\\.tu-darmstadt\\.de/scripts/mgrqispi\\.dll\\?APPNAME=CampusNet&PRGNAME=COURSEDETAILS&ARGUMENTS=-N\\d+,-N\\d+,(.*)$"
    },
    action: {
        type: /** @type {chrome.declarativeNetRequest.RuleActionType} */ ('redirect'),
        redirect: { regexSubstitution: EXT_PAGE + '#/course-details/\\1' },
    },
}, {
    id: 203,
    priority: 3,
    condition: {
        isUrlFilterCaseSensitive: true,
        resourceTypes: [
            /** @type {chrome.declarativeNetRequest.ResourceType} */ ("main_frame")
        ],
        regexFilter: "^https://www\\.tucan\\.tu-darmstadt\\.de/scripts/mgrqispi\\.dll\\?APPNAME=CampusNet&PRGNAME=MODULEDETAILS&ARGUMENTS=-N\\d+,-N\\d+,(.*)$"
    },
    action: {
        type: /** @type {chrome.declarativeNetRequest.RuleActionType} */ ('redirect'),
        redirect: { regexSubstitution: EXT_PAGE + '#/module-details/\\1' },
    },
},];

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