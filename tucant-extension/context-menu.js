import { asyncClosure } from "./utils.js";
import { handleOpenInTucan } from "./open-in-tucan.js"

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
            await chrome.tabs.update(tabId, {
                url: handleOpenInTucan(id?.value, tabId, url)
            })
            return;
        }

        if (info.menuItemId === "open-in-tucan-new-tab" || info.menuItemId === "open-in-tucant-new-tab" || info.menuItemId === "open-in-tucan-page-new-tab" || info.menuItemId === "open-in-tucant-page-new-tab") {
            await chrome.tabs.create({
                url: handleOpenInTucan(id?.value, tabId, url)
            })
            return;
        }


        if (info.menuItemId === "shareable-link-page" || info.menuItemId === "shareable-link") {
            chrome.notifications.create({
                type: "basic",
                iconUrl: chrome.runtime.getURL("/icon-512.png"),
                title: "Sharing this URL is not supported",
                message: "Unfortunately sharing this URL is not supported (yet). We welcome any contribution",
            });
            return;
        }

        chrome.notifications.create({
            type: "basic",
            iconUrl: chrome.runtime.getURL("/icon-512.png"),
            title: "Context menu action not supported",
            message: "Unfortunately this context menu action is not supported yet. We welcome any contribution",
        });
    })
})

const EXTENSION_PAGE = chrome.runtime.getURL('/');

chrome.runtime.onInstalled.addListener(() => {
    asyncClosure(async () => {

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
            id: "open-in-tucan-new-tab",
            title: "Open link in TUCaN in new tab",
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
            id: "open-in-tucant-new-tab",
            title: "Open link in TUCaN't in new tab",
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
            id: "open-in-tucan-page-new-tab",
            title: "Open page in TUCaN in new tab",
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
            id: "open-in-tucant-page-new-tab",
            title: "Open page in TUCaN't in new tab",
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
