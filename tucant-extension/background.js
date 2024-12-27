const EXT_PAGE = chrome.runtime.getURL('/dist/index.html');
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
    console.log(details.transitionQualifiers +
        ' at ' +
        details.transitionType +
        ' milliseconds since the epoch.'
        + details.url
    );
    if (JSON.stringify(details.transitionQualifiers.sort()) === JSON.stringify(["server_redirect", "client_redirect"].sort()) && details.transitionType === "link") {
        const match = new RegExp("^https://www\\.tucan\\.tu-darmstadt\\.de/scripts/mgrqispi\\.dll\\?APPNAME=CampusNet&PRGNAME=MLSSTART&ARGUMENTS=-N(\\d+),-N000019,$", "g").exec(details.url);
        console.log(match)
        if (match !== null) {
            console.log(`logged in with session id ${match[1]}`)
        }
    }
}, { url: [{ urlPrefix: "https://www.tucan.tu-darmstadt.de" }] });