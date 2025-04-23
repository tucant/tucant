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

chrome.cookies.onChanged.addListener(async (changeInfo) => {
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

            if ((await chrome.storage.sync.get(
                { fixSessionIdInUrl: true },
            )).fixSessionIdInUrl) {
                chrome.declarativeNetRequest.updateDynamicRules({
                    removeRuleIds: fixupSessionIdInUrl(changeInfo.cookie.value).map(r => r.id),
                    addRules: fixupSessionIdInUrl(changeInfo.cookie.value),
                });
            }
        }
    }
});

const fixupSessionIdInUrl = (sessionId: string): chrome.declarativeNetRequest.Rule[] => {
    console.log("fixup session id in url")
    return [{
        // redirect any session id to the currently valid session id
        id: 100,
        condition: {
            isUrlFilterCaseSensitive: true,
            resourceTypes: [
                chrome.declarativeNetRequest.ResourceType.MAIN_FRAME
            ],
            regexFilter: `^https://www\\.tucan\\.tu-darmstadt\\.de/scripts/mgrqispi\\.dll\\?APPNAME=CampusNet&PRGNAME=([A-Z_]+)&ARGUMENTS=-N\\d+,(.+)$`
        },
        action: {
            type: chrome.declarativeNetRequest.RuleActionType.REDIRECT,
            redirect: {
                regexSubstitution: `https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=\\1&ARGUMENTS=-N${sessionId},\\2`,
            },
        },
    }, {
        // but don't create an infinite loop
        id: 101,
        priority: 2,
        condition: {
            isUrlFilterCaseSensitive: true,
            resourceTypes: [
                chrome.declarativeNetRequest.ResourceType.MAIN_FRAME
            ],
            regexFilter: `^https://www\\.tucan\\.tu-darmstadt\\.de/scripts/mgrqispi\\.dll\\?APPNAME=CampusNet&PRGNAME=([A-Z_]+)&ARGUMENTS=-N${sessionId},(.+)$`
        },
        action: {
            type: chrome.declarativeNetRequest.RuleActionType.ALLOW
        },
    }, {
        // and don't redirect explicitly unauthenticated urls
        id: 102,
        priority: 2,
        condition: {
            isUrlFilterCaseSensitive: true,
            resourceTypes: [
                chrome.declarativeNetRequest.ResourceType.MAIN_FRAME
            ],
            regexFilter: `^https://www\\.tucan\\.tu-darmstadt\\.de/scripts/mgrqispi\\.dll\\?APPNAME=CampusNet&PRGNAME=([A-Z_]+)&ARGUMENTS=-N000000000000001,(.+)$`
        },
        action: {
            type: chrome.declarativeNetRequest.RuleActionType.ALLOW
        },
    }, {
        id: 103,
        priority: 2,
        condition: {
            isUrlFilterCaseSensitive: true,
            resourceTypes: [
                chrome.declarativeNetRequest.ResourceType.MAIN_FRAME
            ],
            regexFilter: `^https://www\\.tucan\\.tu-darmstadt\\.de/$`
        },
        action: {
            type: chrome.declarativeNetRequest.RuleActionType.REDIRECT,
            redirect: {
                regexSubstitution: `https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=MLSSTART&ARGUMENTS=-N${sessionId},-N000019,`,
            },
        },
    }]
};

chrome.storage.sync.onChanged.addListener(async (changes) => {
    for (let [key, { oldValue, newValue }] of Object.entries(changes)) {
        if (key === "fixSessionIdInUrl") {
            if (newValue) {
                const id = await chrome.cookies.get({
                    url: "https://www.tucan.tu-darmstadt.de/scripts/",
                    name: "id",
                })
                if (id) {
                    chrome.declarativeNetRequest.updateDynamicRules({
                        removeRuleIds: fixupSessionIdInUrl(id.value).map(r => r.id),
                        addRules: fixupSessionIdInUrl(id.value),
                    });
                } else {
                    chrome.declarativeNetRequest.updateDynamicRules({
                        removeRuleIds: fixupSessionIdInUrl("").map(r => r.id),
                        addRules: [],
                    });
                }
            } else {
                chrome.declarativeNetRequest.updateDynamicRules({
                    removeRuleIds: fixupSessionIdInUrl("").map(r => r.id),
                    addRules: [],
                });
            }
        }
    }
});

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