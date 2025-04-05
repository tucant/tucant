document.querySelector('#go-to-options').addEventListener('click', function () {
    if (chrome.runtime.openOptionsPage) {
        chrome.runtime.openOptionsPage();
    } else {
        window.open(chrome.runtime.getURL('options.html'));
    }
});

const EXTENSION_PAGE = chrome.runtime.getURL('/');

document.querySelector("#update-extension")?.addEventListener('click', async function () {
    console.log("test")
    // https://gist.github.com/NiklasGollenstede/63a6099d97e82ffe0cc064d4d4d82b62

    let tabs = await chrome.tabs.query({
        url: `${EXTENSION_PAGE}*`
    })

    tabs.forEach(tab => {
        chrome.tabs.discard(tab.id)
    })

    chrome.runtime.reload();
})

document.querySelector('#grant-permission').addEventListener('click', async (event) => {
    if (await chrome.permissions.request({
        origins: ['https://www.tucan.tu-darmstadt.de/', 'http://www.tucan.tu-darmstadt.de/']
    })) {
        document.querySelector("#grant-permission-area").style.display = "none";
    }
});

if (!await chrome.permissions.contains({
    origins: ['https://www.tucan.tu-darmstadt.de/', 'http://www.tucan.tu-darmstadt.de/']
})) {
    console.log("no host permissions")
    document.querySelector("#grant-permission-area").style.display = "block";
} else {
    console.log("have host permission")
}