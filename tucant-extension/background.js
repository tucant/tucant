import "./fix-session-id-in-url.js"
import { handleOpenInTucan, getCurrentTab } from "./open-in-tucan.js"
import { asyncClosure } from "./utils.js";

console.log("background script")

const EXTENSION_DOMAIN = chrome.runtime.getURL('');
const EXTENSION_PAGE = chrome.runtime.getURL('/');

chrome.runtime.onMessage.addListener((message, sender) => {
    asyncClosure(async () => {
        console.log("onMessage", message, sender)

        chrome.notifications.create({
            type: "basic",
            iconUrl: chrome.runtime.getURL("/icon-512.png"),
            title: "TUCaN't extension message",
            message: String(message),
        });

        if (!sender.tab?.id || !sender.tab.url) {
            console.log("no tab id or url")
            return;
        }

        if (message === "open-in-tucan-page") {
            const id = await chrome.cookies.get({
                url: "https://www.tucan.tu-darmstadt.de/scripts/",
                name: "id",
            })

            await handleOpenInTucan(id?.value, sender.tab.id, sender.tab.url)
        }
    })
})

chrome.commands.onCommand.addListener((command) => {
    asyncClosure(async () => {
        const id = await chrome.cookies.get({
            url: "https://www.tucan.tu-darmstadt.de/scripts/",
            name: "id",
        })

        let tab = await getCurrentTab()

        if (!tab.id || !tab.url) {
            console.log("no tab id or url")
            return;
        }

        if (command === "open-in-tucan-page") {
            await handleOpenInTucan(id?.value, tab.id, tab.url)
        }
    })
});

chrome.contextMenus.onClicked.addListener((info, tab) => {
    asyncClosure(async () => {
        const id = await chrome.cookies.get({
            url: "https://www.tucan.tu-darmstadt.de/scripts/",
            name: "id",
        })

        let url = info.linkUrl ?? info.pageUrl
        let tabId = tab?.id

        if (!tabId) {
            return;
        }

        if (info.menuItemId === "open-in-tucan" || info.menuItemId === "open-in-tucant" || info.menuItemId === "open-in-tucan-page" || info.menuItemId === "open-in-tucant-page") {
            await handleOpenInTucan(id?.value, tabId, url)
        }

        if (info.menuItemId === "shareable-link-page" || info.menuItemId === "shareable-link") {
            chrome.notifications.create({
                type: "basic",
                iconUrl: chrome.runtime.getURL("/icon-512.png"),
                title: "Sharing this URL is not supported",
                message: "Unfortunately sharing this URL is not supported (yet). We welcome any contribution",
            });
        }
    })
})

chrome.runtime.onInstalled.addListener(() => {
    asyncClosure(async () => {
        console.log("oninstalled")
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

        console.log("enable selfmade4u rule")
        const rules = {
            removeRuleIds: [4100], // TODO check that rules have no dupes
            addRules: [{
                id: 4100,
                priority: 10,
                condition: {
                    isUrlFilterCaseSensitive: true,
                    resourceTypes: [
/** @type {chrome.declarativeNetRequest.ResourceType} */ ("main_frame")
                    ],
                    urlFilter: `|https://tucant.selfmade4u.de/*`
                },
                action: {
                    type: /** @type {chrome.declarativeNetRequest.RuleActionType} */ ('redirect'),
                    redirect: {
                        // I think this needs to statically be an allowed url
                        transform: {
                            scheme: EXTENSION_DOMAIN.split("://")[0],
                            host: EXTENSION_DOMAIN.slice(0, -1).replace("moz-extension://", "").replace("chrome-extension://", "")
                        }
                    },
                },
            }],
        }
        console.log(rules)
        await chrome.declarativeNetRequest.updateDynamicRules(rules);
        console.log(chrome.runtime.lastError)

        let tabs = await chrome.tabs.query({
            url: `https://tucant.selfmade4u.de/*`
        })

        await Promise.all(tabs.map(async tab => {
            if (tab.id) {
                await chrome.tabs.reload(tab.id)
            }
        }))

        await chrome.contextMenus.removeAll();

        chrome.contextMenus.create({
            id: "open-in-tucan",
            title: "Open link in TUCaN",
            contexts: ["link"],
            targetUrlPatterns: [`${EXTENSION_PAGE}*`]
        }, () => {
            console.log(chrome.runtime.lastError)
        })

        chrome.contextMenus.create({
            id: "open-in-tucant",
            title: "Open link in TUCaN't",
            contexts: ["link"],
            targetUrlPatterns: ["https://www.tucan.tu-darmstadt.de/*"]
        }, () => {
            console.log(chrome.runtime.lastError)
        })

        chrome.contextMenus.create({
            id: "open-in-tucan-page",
            title: "Open page in TUCaN",
            contexts: ["page"],
            documentUrlPatterns: [`${EXTENSION_PAGE}*`]
        }, () => {
            console.log(chrome.runtime.lastError)
        })

        chrome.contextMenus.create({
            id: "open-in-tucant-page",
            title: "Open page in TUCaN't",
            contexts: ["page"],
            documentUrlPatterns: ["https://www.tucan.tu-darmstadt.de/*"]
        }, () => {
            console.log(chrome.runtime.lastError)
        })

        chrome.contextMenus.create({
            id: "shareable-link-page",
            title: "Share link to page (without session id)",
            contexts: ["page"],
            documentUrlPatterns: ["https://www.tucan.tu-darmstadt.de/*", `${EXTENSION_PAGE}*`]
        }, () => {
            console.log(chrome.runtime.lastError)
        })

        chrome.contextMenus.create({
            id: "shareable-link",
            title: "Share link (without session id)",
            contexts: ["link"],
            documentUrlPatterns: ["https://www.tucan.tu-darmstadt.de/*", `${EXTENSION_PAGE}*`]
        }, () => {
            console.log(chrome.runtime.lastError)
        })

    });
});

chrome.storage.sync.onChanged.addListener((changes) => {
    asyncClosure(async () => {
        for (let [key, { newValue }] of Object.entries(changes)) {
            if (key === "mobileDesign") {
                if (newValue) {
                    await enableMobileDesign()
                } else {
                    await disableMobileDesign()
                }
            }
            if (key === "customUi") {
                if (newValue) {
                    await enableCustomUi()
                } else {
                    await disableCustomUi()
                }
            }
        }
    });
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
}, {
    id: 204,
    priority: 3,
    condition: {
        isUrlFilterCaseSensitive: true,
        resourceTypes: [
            /** @type {chrome.declarativeNetRequest.ResourceType} */ ("main_frame")
        ],
        regexFilter: "^https://www\\.tucan\\.tu-darmstadt\\.de/scripts/mgrqispi\\.dll\\?APPNAME=CampusNet&PRGNAME=MLSSTART&ARGUMENTS=-N\\d+,-N\\d+,$",
        excludedInitiatorDomains: [EXTENSION_DOMAIN.slice(0, -1).replace("moz-extension://", "").replace("chrome-extension://", "")]
    },
    action: {
        type: /** @type {chrome.declarativeNetRequest.RuleActionType} */ ('redirect'),
        redirect: { regexSubstitution: EXT_PAGE + '#/overview' },
    },
},
// TODO how do we handle ACTION urls that are not clearly detectable?

// TODO selected semester
{
    id: 205,
    priority: 3,
    condition: {
        isUrlFilterCaseSensitive: true,
        resourceTypes: [
            /** @type {chrome.declarativeNetRequest.ResourceType} */ ("main_frame")
        ],
        regexFilter: "^https://www\\.tucan\\.tu-darmstadt\\.de/scripts/mgrqispi\\.dll\\?APPNAME=CampusNet&PRGNAME=MYMODULES&ARGUMENTS=-N\\d+,-N\\d+,$",
        excludedInitiatorDomains: [EXTENSION_DOMAIN.slice(0, -1).replace("moz-extension://", "").replace("chrome-extension://", "")]
    },
    action: {
        type: /** @type {chrome.declarativeNetRequest.RuleActionType} */ ('redirect'),
        redirect: { regexSubstitution: EXT_PAGE + '#/my-modules/current' },
    },
},
{
    id: 206,
    priority: 3,
    condition: {
        isUrlFilterCaseSensitive: true,
        resourceTypes: [
            /** @type {chrome.declarativeNetRequest.ResourceType} */ ("main_frame")
        ],
        regexFilter: "^https://www\\.tucan\\.tu-darmstadt\\.de/scripts/mgrqispi\\.dll\\?APPNAME=CampusNet&PRGNAME=PROFCOURSES&ARGUMENTS=-N\\d+,-N\\d+,$",
        excludedInitiatorDomains: [EXTENSION_DOMAIN.slice(0, -1).replace("moz-extension://", "").replace("chrome-extension://", "")]
    },
    action: {
        type: /** @type {chrome.declarativeNetRequest.RuleActionType} */ ('redirect'),
        redirect: { regexSubstitution: EXT_PAGE + '#/my-courses/current' },
    },
},
{
    id: 207,
    priority: 3,
    condition: {
        isUrlFilterCaseSensitive: true,
        resourceTypes: [
            /** @type {chrome.declarativeNetRequest.ResourceType} */ ("main_frame")
        ],
        regexFilter: "^https://www\\.tucan\\.tu-darmstadt\\.de/scripts/mgrqispi\\.dll\\?APPNAME=CampusNet&PRGNAME=MYEXAMS&ARGUMENTS=-N\\d+,-N\\d+,$",
        excludedInitiatorDomains: [EXTENSION_DOMAIN.slice(0, -1).replace("moz-extension://", "").replace("chrome-extension://", "")]
    },
    action: {
        type: /** @type {chrome.declarativeNetRequest.RuleActionType} */ ('redirect'),
        redirect: { regexSubstitution: EXT_PAGE + '#/my-exams/current' },
    },
},
{
    id: 208,
    priority: 3,
    condition: {
        isUrlFilterCaseSensitive: true,
        resourceTypes: [
            /** @type {chrome.declarativeNetRequest.ResourceType} */ ("main_frame")
        ],
        regexFilter: "^https://www\\.tucan\\.tu-darmstadt\\.de/scripts/mgrqispi\\.dll\\?APPNAME=CampusNet&PRGNAME=COURSERESULTS&ARGUMENTS=-N\\d+,-N\\d+,$",
        excludedInitiatorDomains: [EXTENSION_DOMAIN.slice(0, -1).replace("moz-extension://", "").replace("chrome-extension://", "")]
    },
    action: {
        type: /** @type {chrome.declarativeNetRequest.RuleActionType} */ ('redirect'),
        redirect: { regexSubstitution: EXT_PAGE + '#/course-results/current' },
    },
},
{
    id: 209,
    priority: 3,
    condition: {
        isUrlFilterCaseSensitive: true,
        resourceTypes: [
            /** @type {chrome.declarativeNetRequest.ResourceType} */ ("main_frame")
        ],
        regexFilter: "^https://www\\.tucan\\.tu-darmstadt\\.de/scripts/mgrqispi\\.dll\\?APPNAME=CampusNet&PRGNAME=EXAMRESULTS&ARGUMENTS=-N\\d+,-N\\d+,$",
        excludedInitiatorDomains: [EXTENSION_DOMAIN.slice(0, -1).replace("moz-extension://", "").replace("chrome-extension://", "")]
    },
    action: {
        type: /** @type {chrome.declarativeNetRequest.RuleActionType} */ ('redirect'),
        redirect: { regexSubstitution: EXT_PAGE + '#/exam-results/current' },
    },
},
{
    id: 210,
    priority: 3,
    condition: {
        isUrlFilterCaseSensitive: true,
        resourceTypes: [
            /** @type {chrome.declarativeNetRequest.ResourceType} */ ("main_frame")
        ],
        regexFilter: "^https://www\\.tucan\\.tu-darmstadt\\.de/scripts/mgrqispi\\.dll\\?APPNAME=CampusNet&PRGNAME=STUDENT_RESULT&ARGUMENTS=-N\\d+,-N\\d+,-N0,-N000000000000000,-N000000000000000,-N000000000000000,-N0,-N000000000000000$",
        excludedInitiatorDomains: [EXTENSION_DOMAIN.slice(0, -1).replace("moz-extension://", "").replace("chrome-extension://", "")]
    },
    action: {
        type: /** @type {chrome.declarativeNetRequest.RuleActionType} */ ('redirect'),
        redirect: { regexSubstitution: EXT_PAGE + '#/student-result/default' },
    },
},
{
    id: 211,
    priority: 3,
    condition: {
        isUrlFilterCaseSensitive: true,
        resourceTypes: [
            /** @type {chrome.declarativeNetRequest.ResourceType} */ ("main_frame")
        ],
        regexFilter: "^https://www\\.tucan\\.tu-darmstadt\\.de/scripts/mgrqispi\\.dll\\?APPNAME=CampusNet&PRGNAME=CREATEDOCUMENT&ARGUMENTS=-N\\d+,-N\\d+,$",
        excludedInitiatorDomains: [EXTENSION_DOMAIN.slice(0, -1).replace("moz-extension://", "").replace("chrome-extension://", "")]
    },
    action: {
        type: /** @type {chrome.declarativeNetRequest.RuleActionType} */ ('redirect'),
        redirect: { regexSubstitution: EXT_PAGE + '#/my-documents' },
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

chrome.omnibox.onInputChanged.addListener(() => {
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

chrome.omnibox.onInputEntered.addListener(() => {
    asyncClosure(async () => {
        await chrome.tabs.update({ url: "https://www.tucan.tu-darmstadt.de" })
    })
})

export { }