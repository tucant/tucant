import { bidirectionalMappings } from "./url-mappings";

const EXT_PAGE_INDEX_HTML = chrome.runtime.getURL('/public/index.html');

export async function getCurrentTab() {
    let queryOptions = { active: true, lastFocusedWindow: true };
    // `tab` will either be a `tabs.Tab` instance or `undefined`.
    // TODO FIXME typescript is wrong here
    let [tab] = await chrome.tabs.query(queryOptions);
    return tab;
}

/**
 * @param {string | undefined} id
 * @param {number} tabId
 * @param {string} url
 * @returns {Promise<string|undefined>}
 */
export async function handleOpenInTucan(id, tabId, url) {
    const mappings = bidirectionalMappings(id);
    mappings.forEach(mapping => {
        const regex = mapping.tucan.strings.reduce((acc, curr, i) => {
            const substitution = mapping.tucan.args[i];
            return (acc += substitution
                ? `${RegExp.escape(curr)}${substitution.from}`
                : curr);
        }, '^https://www\\.tucan\\.tu-darmstadt\\.de/scripts/mgrqispi\\.dll\\?APPNAME=CampusNet&') + '$';
        let replacementIdx = 1;
        const replacement = mapping.tucan.strings.reduce((acc, curr, i) => {
            const substitution = mapping.tucan.args[i];
            return (acc += substitution
                ? `${RegExp.escape(curr)}${substitution.to ?? (replacementIdx++).toString()}`
                : curr);
        }, `${EXT_PAGE_INDEX_HTML}#/`) + '$';
        console.log(regex)
        console.log(replacement)

        let match = new RegExp(regex, "g").exec(url)
        if (match) {
            return url.replace(regex, replacement)
        }
    })

    if (!id) {
        await chrome.notifications.create({
            type: "basic",
            iconUrl: chrome.runtime.getURL("/icon-512.png"),
            title: "Not logged in",
            message: "Could not detect session, please login again",
        });
        return undefined;
    }

    await chrome.notifications.create({
        type: "basic",
        iconUrl: chrome.runtime.getURL("/icon-512.png"),
        title: "URL not supported",
        message: "Unfortunately this URL is not supported yet. We welcome any contribution",
    });
    return undefined;
}
