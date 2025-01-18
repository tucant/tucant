document.querySelector('#go-to-options').addEventListener('click', function () {
    if (chrome.runtime.openOptionsPage) {
        chrome.runtime.openOptionsPage();
    } else {
        window.open(chrome.runtime.getURL('options.html'));
    }
});

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