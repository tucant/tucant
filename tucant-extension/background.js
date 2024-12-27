const EXT_PAGE = chrome.runtime.getURL('/dist/index.html');
/** @type {chrome.declarativeNetRequest.Rule[]} */
const RULES = [{
    id: 1337,
    action: {
        type: 'redirect',
        redirect: { regexSubstitution: EXT_PAGE + '#\\0' },
    },
    "condition": {
        "isUrlFilterCaseSensitive": true,
        "resourceTypes": [
            "main_frame"
        ],
        "regexFilter": "^https://www\\.tucan\\.tu-darmstadt\\.de/scripts/mgrqispi\\.dll\\?APPNAME=CampusNet&PRGNAME=REGISTRATION&ARGUMENTS=-N(\\d+),-N000311,-A$"
    }
}];
chrome.declarativeNetRequest.updateDynamicRules({
    removeRuleIds: RULES.map(r => r.id),
    addRules: RULES,
});

chrome.webNavigation.onCommitted.addListener((details) => {
    if (JSON.stringify(details.transitionQualifiers.sort()) === JSON.stringify(["server_redirect", "client_redirect"].sort()) && details.transitionType === "link") {
        const match = new RegExp("^https://www\\.tucan\\.tu-darmstadt\\.de/scripts/mgrqispi\\.dll\\?APPNAME=CampusNet&PRGNAME=MLSSTART&ARGUMENTS=-N(\\d+),-N000019,$", "g").exec(details.url);
        if (match !== null) {
            const sessionId = match[1]
            console.log(`logged in with session id ${sessionId}`);
            chrome.action.setBadgeText({ text: "L" })
            chrome.action.setBadgeBackgroundColor(
                { color: 'green' }
            )
            chrome.action.setBadgeTextColor({ color: "white" });

            // N628581235999313
            // N843473130956881
            // N208504055404716
            // N486415145950940
            //  000000000000001
            // https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&PRGNAME=COURSEDETAILS&ARGUMENTS=-N166632378335734,-N000274,-N380005141348847,-N388203828671910,-N388203828624911,-N0,-N0

            /** @type {chrome.declarativeNetRequest.Rule[]} */
            const RULES = [{
                id: 1338,
                action: {
                    type: 'redirect',
                    redirect: { regexSubstitution: "https://selfmade4u.de/" + '#\\0' },
                },
                "condition": {
                    "isUrlFilterCaseSensitive": true,
                    "resourceTypes": [
                        "main_frame"
                    ],
                    "regexFilter": `^https://www\\.tucan\\.tu-darmstadt\\.de/scripts/mgrqispi\\.dll\\?APPNAME=CampusNet&PRGNAME=([A-Z]+)&ARGUMENTS=-N(\\d+),(.+)$`
                }
            }, {
                id: 1339,
                priority: 2,
                action: {
                    type: 'allow'
                },
                "condition": {
                    "isUrlFilterCaseSensitive": true,
                    "resourceTypes": [
                        "main_frame"
                    ],
                    "regexFilter": `^https://www\\.tucan\\.tu-darmstadt\\.de/scripts/mgrqispi\\.dll\\?APPNAME=CampusNet&PRGNAME=([A-Z]+)&ARGUMENTS=-N${sessionId},(.+)$`
                }
            }];
            chrome.declarativeNetRequest.updateDynamicRules({
                removeRuleIds: RULES.map(r => r.id),
                addRules: RULES,
            });
        }

        const logoutMatch = new RegExp("^https://www\\.tucan\\.tu-darmstadt\\.de/scripts/mgrqispi\\.dll\\?APPNAME=CampusNet&PRGNAME=EXTERNALPAGES&ARGUMENTS=-N000000000000001,-N000344,-Awelcome$", "g").exec(details.url);
        if (logoutMatch !== null) {
            console.log(`logged out`);
            chrome.action.setBadgeText({ text: "" })
        }
    }


}, { url: [{ urlPrefix: "https://www.tucan.tu-darmstadt.de" }] });
