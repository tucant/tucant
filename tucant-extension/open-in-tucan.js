import { bidirectionalMappings } from "./url-mappings.js";

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

    for (const mapping of mappings) {
        const regex = new RegExp(mapping.tucan.strings.reduce((acc, curr, i) => {
            const substitution = mapping.tucan.args[i];
            return (acc += i < mapping.tucan.args.length
                ? `${RegExp.escape(curr)}${substitution.from}`
                : RegExp.escape(curr));
        }, '^https://www\\.tucan\\.tu-darmstadt\\.de/scripts/mgrqispi\\.dll\\?APPNAME=CampusNet&') + '$', "g");
        let replacementIdx = 1;
        const replacement = mapping.tucant.strings.reduce((acc, curr, i) => {
            const substitution = mapping.tucant.args[i];
            return (acc += i < mapping.tucant.args.length
                ? `${curr}${substitution.to ?? `$${(replacementIdx++).toString()}`}`
                : curr);
        }, `${EXT_PAGE_INDEX_HTML}#/`);
        console.log(regex)
        console.log(replacement)

        let match = regex.exec(url)
        if (match) {
            let result = url.replace(regex, replacement)
            console.log("result ", result)
            return result
        }
    }

    for (const mapping of mappings) {
        const regex = new RegExp(mapping.tucant.strings.reduce((acc, curr, i) => {
            const substitution = mapping.tucant.args[i];
            return (acc += i < mapping.tucant.args.length
                ? `${RegExp.escape(curr)}${substitution.from}`
                : RegExp.escape(curr));
        }, `^${EXT_PAGE_INDEX_HTML}#/`) + '$', "g");
        let replacementIdx = 1;
        const replacement = mapping.tucan.strings.reduce((acc, curr, i) => {
            const substitution = mapping.tucan.args[i];
            return (acc += i < mapping.tucan.args.length
                ? `${curr}${substitution.to ?? `$${(replacementIdx++).toString()}`}`
                : curr);
        }, `https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll?APPNAME=CampusNet&`);
        console.log(regex)
        console.log(replacement)

        let match = regex.exec(url)
        if (match) {
            let result = url.replace(regex, replacement)
            console.log("result ", result)
            return result
        }
    }

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
