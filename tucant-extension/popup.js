import { asyncClosure } from "./utils.js";

/** @type {HTMLElement} */ (document.querySelector('#go-to-options')).addEventListener('click', function () {
    asyncClosure(async () => {
        await chrome.runtime.openOptionsPage();
    })
});

const EXT_PAGE_INDEX_HTML = chrome.runtime.getURL('/dist/index.html');

/** @type {HTMLElement} */ (document.querySelector('#open-custom-ui')).addEventListener('click', function () {
    asyncClosure(async () => {
        await chrome.tabs.create({
            url: EXT_PAGE_INDEX_HTML
        })
    });
})

// TODO maybe chrome.runtime.onUpdateAvailable

document.querySelector("#update-extension")?.addEventListener('click', function () {
    asyncClosure(async () => {
        console.log("update extension")

        // Chrome will close all extension tabs including blob urls, see https://issues.chromium.org/issues/41189391
        // The following is a hack and should mostly be used for development

        // https://stackoverflow.com/questions/68422688/chrome-extension-declarativenetrequest-isnt-matching-rulecondition

        console.log("remove selfmade4u rule")
        await chrome.declarativeNetRequest.updateDynamicRules({
            // TODO centrally reference this id
            removeRuleIds: [4100], // TODO check that rules have no dupes
        });

        // https://issues.chromium.org/issues/40670457
        let tabs = await chrome.runtime.getContexts({
            contextTypes: [/** @type {chrome.runtime.ContextType.TAB} */("TAB")],
        });

        await Promise.all(tabs.map(tab => {
            if (!tab.documentUrl) {
                return;
            }
            let url = new URL(tab.documentUrl);
            return chrome.tabs.update(tab.tabId, {
                url: "https://tucant.selfmade4u.de/" + url.hash
            })
        }));

        await new Promise(r => setTimeout(r, 500));

        chrome.runtime.reload();
    })
});

/** @type {HTMLElement} */(document.querySelector('#grant-permission')).addEventListener('click', () => {
    asyncClosure(async () => {
        if (await chrome.permissions.request({
            origins: ['https://www.tucan.tu-darmstadt.de/', 'http://www.tucan.tu-darmstadt.de/', 'https://tucant.selfmade4u.de/']
        })) {
        /** @type {HTMLElement} */ (document.querySelector("#grant-permission-area")).style.display = "none";
        }
    })
});

if (!await chrome.permissions.contains({
    origins: ['https://www.tucan.tu-darmstadt.de/', 'http://www.tucan.tu-darmstadt.de/', 'https://tucant.selfmade4u.de/']
})) {
    console.log("no host permissions");
    /** @type {HTMLElement} */(document.querySelector("#grant-permission-area")).style.display = "block";
} else {
    console.log("have host permission");
}

export { }