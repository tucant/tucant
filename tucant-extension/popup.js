document.querySelector('#go-to-options').addEventListener('click', function () {
    if (chrome.runtime.openOptionsPage) {
        chrome.runtime.openOptionsPage();
    } else {
        window.open(chrome.runtime.getURL('options.html'));
    }
});

const EXTENSION_PAGE = chrome.runtime.getURL('/');

const EXT_PAGE_INDEX_HTML = chrome.runtime.getURL('/dist/index.html');

// TODO maybe chrome.runtime.onUpdateAvailable

document.querySelector("#update-extension")?.addEventListener('click', async function () {
    console.log("test")

    // Chrome will close all extension tabs including blob urls, see https://issues.chromium.org/issues/41189391
    // The following is a hack and should mostly be used for development

    await chrome.declarativeNetRequest.updateDynamicRules({
        removeRuleIds: [4100], // TODO check that rules have no dupes
        addRules: [{
            id: 4100,
            action: {
                type: /** @type {chrome.declarativeNetRequest.RuleActionType} */ ('redirect'),
                redirect: {
                    // I think this needs to statically be an allowed url
                    regexSubstitution: `https://tucant.selfmade4u.de/#\\1`,
                },
            },
            condition: {
                isUrlFilterCaseSensitive: true,
                resourceTypes: [
                    /** @type {chrome.declarativeNetRequest.ResourceType} */ ("main_frame")
                ],
                regexFilter: `^${EXT_PAGE_INDEX_HTML}#(.*)$`
            }
        }],
    });

    await new Promise(r => setTimeout(r, 500));

    // https://issues.chromium.org/issues/40670457
    let tabs = await chrome.runtime.getContexts({
        contextTypes: [/** @type {chrome.runtime.ContextType.TAB} */ ("TAB")],
    })

    console.log("tabs", tabs)

    await Promise.all(tabs.map(tab => {
        return chrome.tabs.reload(tab.tabId)
    }))

    await new Promise(r => setTimeout(r, 500));

    chrome.runtime.reload();
})

document.querySelector('#grant-permission').addEventListener('click', async (event) => {
    if (await chrome.permissions.request({
        origins: ['https://www.tucan.tu-darmstadt.de/', 'http://www.tucan.tu-darmstadt.de/', 'https://tucant.selfmade4u.de/']
    })) {
        document.querySelector("#grant-permission-area").style.display = "none";
    }
});

if (!await chrome.permissions.contains({
    origins: ['https://www.tucan.tu-darmstadt.de/', 'http://www.tucan.tu-darmstadt.de/', 'https://tucant.selfmade4u.de/']
})) {
    console.log("no host permissions")
    document.querySelector("#grant-permission-area").style.display = "block";
} else {
    console.log("have host permission")
}