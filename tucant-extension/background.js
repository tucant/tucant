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
    chrome.notifications.create({
        type: 'basic',
        iconUrl: 'icon.png',
        title: 'page loaded',
        message:
            details.transitionQualifiers +
            ' at ' +
            details.transitionType +
            ' milliseconds since the epoch.'
    });
}, { url: [{ urlPrefix: "https://www.tucan.tu-darmstadt.de" }] });