import "./fix-session-id-in-url.js"
import { handleOpenInTucan } from "./open-in-tucan.js"

console.log("background script")

const EXTENSION_DOMAIN = chrome.runtime.getURL('');
const EXTENSION_PAGE = chrome.runtime.getURL('/');
const EXT_PAGE_INDEX_HTML = chrome.runtime.getURL('/dist/index.html');

async function getCurrentTab() {
    let queryOptions = { active: true, lastFocusedWindow: true };
    // `tab` will either be a `tabs.Tab` instance or `undefined`.
    // TODO FIXME typescript is wrong here
    let [tab] = await chrome.tabs.query(queryOptions);
    return tab;
}

chrome.commands.onCommand.addListener(async (command) => {
    console.log("handlecommand")
    const id = await chrome.cookies.get({
        url: "https://www.tucan.tu-darmstadt.de/scripts/",
        name: "id",
    })

    let tab = await getCurrentTab()

    if (!tab?.id || !tab.url) {
        console.log("no tab id or url")
        return;
    }

    if (command === "open-in-tucan-page") {
        console.log("opefwewf")
        handleOpenInTucan(id?.value, tab.id, tab.url)
    }
});

chrome.contextMenus.onClicked.addListener(async (info, tab) => {
    const id = await chrome.cookies.get({
        url: "https://www.tucan.tu-darmstadt.de/scripts/",
        name: "id",
    })

    let url = info.linkUrl || info.pageUrl
    let tabId = tab?.id

    if (!tabId) {
        return;
    }

    handleOpenInTucan(id?.value, tabId, url)
})

chrome.runtime.onInstalled.addListener(async () => {
    let { mobileDesign, customUi } = await chrome.storage.sync.get(
        { mobileDesign: false, customUi: true },
    );

    if (customUi) {
        await enableCustomUi()
    } else {
        await disableCustomUi()
    }

    if (mobileDesign) {
        await enableMobileDesign()
    } else {
        await disableMobileDesign()
    }

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
/*
chrome.declarativeNetRequest.onRuleMatchedDebug.addListener(
    event => {
        console.log(event)
    }
)
*/
// https://groups.google.com/a/chromium.org/g/chromium-extensions/c/v3yrOjZIDJc
const EXT_PAGE = chrome.runtime.getURL('/dist/index.html');
console.log(EXTENSION_DOMAIN.slice(0, -1).replace("moz-extension://", "").replace("chrome-extension://", ""))
/** @type {chrome.declarativeNetRequest.Rule[]} */
const customUiRules = [{
    id: 200,
    priority: 3,
    condition: {
        isUrlFilterCaseSensitive: true,
        resourceTypes: [
            /** @type {chrome.declarativeNetRequest.ResourceType} */ ("main_frame")
        ],
        regexFilter: "^https://www\\.tucan\\.tu-darmstadt\\.de/$",
        excludedInitiatorDomains: [EXTENSION_DOMAIN.slice(0, -1).replace("moz-extension://", "").replace("chrome-extension://", "")]
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
        regexFilter: "^https://www\\.tucan\\.tu-darmstadt\\.de/scripts/mgrqispi\\.dll\\?APPNAME=CampusNet&PRGNAME=REGISTRATION&ARGUMENTS=-N\\d+,-N\\d+,(.*)$",
        excludedInitiatorDomains: [EXTENSION_DOMAIN.slice(0, -1).replace("moz-extension://", "").replace("chrome-extension://", "")]
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
        regexFilter: "^https://www\\.tucan\\.tu-darmstadt\\.de/scripts/mgrqispi\\.dll\\?APPNAME=CampusNet&PRGNAME=COURSEDETAILS&ARGUMENTS=-N\\d+,-N\\d+,(.*)$",
        excludedInitiatorDomains: [EXTENSION_DOMAIN.slice(0, -1).replace("moz-extension://", "").replace("chrome-extension://", "")]
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
        regexFilter: "^https://www\\.tucan\\.tu-darmstadt\\.de/scripts/mgrqispi\\.dll\\?APPNAME=CampusNet&PRGNAME=MODULEDETAILS&ARGUMENTS=-N\\d+,-N\\d+,(.*)$",
        excludedInitiatorDomains: [EXTENSION_DOMAIN.slice(0, -1).replace("moz-extension://", "").replace("chrome-extension://", "")]
    },
    action: {
        type: /** @type {chrome.declarativeNetRequest.RuleActionType} */ ('redirect'),
        redirect: { regexSubstitution: EXT_PAGE + '#/module-details/\\1' },
    },
},];

async function enableCustomUi() {
    await chrome.declarativeNetRequest.updateDynamicRules({
        removeRuleIds: customUiRules.map(r => r.id),
        addRules: customUiRules,
    })
}

async function disableCustomUi() {
    await chrome.declarativeNetRequest.updateDynamicRules({
        removeRuleIds: customUiRules.map(r => r.id)
    })
}

async function enableMobileDesign() {
    await chrome.scripting.registerContentScripts(
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

async function disableMobileDesign() {
    const registeredContentScripts = await chrome.scripting.getRegisteredContentScripts()
    if (registeredContentScripts.find(s => s.id === "mobile")) {
        await chrome.scripting.unregisterContentScripts({
            ids: ["mobile"]
        })
    }
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

export { }