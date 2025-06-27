import { bidirectionalMappings } from "./url-mappings.js";

const EXTENSION_DOMAIN = chrome.runtime.getURL('');
const EXT_PAGE_INDEX_HTML = chrome.runtime.getURL('/public/index.html');

const mappings = bidirectionalMappings("42");

const rules = [...mappings].map((mapping, index) => {
    const regex = mapping.tucan.strings.reduce((acc, curr, i) => {
        const substitution = mapping.tucan.args[i];
        return (acc += i < mapping.tucan.args.length
            ? `${RegExp.escape(curr)}${substitution.from}`
            : RegExp.escape(curr));
    }, '^https://www\\.tucan\\.tu-darmstadt\\.de/scripts/mgrqispi\\.dll\\?APPNAME=CampusNet&') + '$';
    let replacementIdx = 1;
    const replacement = mapping.tucant.strings.reduce((acc, curr, i) => {
        const substitution = mapping.tucant.args[i];
        return (acc += i < mapping.tucant.args.length
            ? `${curr}${substitution.to ?? `\\${(replacementIdx++).toString()}`}`
            : curr);
    }, `${EXT_PAGE_INDEX_HTML}#/`);

    // excludedTabIds
    return {
        id: 201 + index,
        priority: 3,
        condition: {
            isUrlFilterCaseSensitive: true,
            resourceTypes: [
            /** @type {chrome.declarativeNetRequest.ResourceType} */ ("main_frame")
            ],
            regexFilter: regex,
            excludedInitiatorDomains: [EXTENSION_DOMAIN.slice(0, -1).replace("moz-extension://", "").replace("chrome-extension://", "")]
        },
        action: {
            type: /** @type {chrome.declarativeNetRequest.RuleActionType} */ ('redirect'),
            redirect: { regexSubstitution: replacement },
        },
    };
})

/** @type {chrome.declarativeNetRequest.Rule[]} */
export const customUiRules = [{
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
        redirect: { regexSubstitution: EXT_PAGE_INDEX_HTML + '#/' },
    },
}, ...rules];

console.log(customUiRules)