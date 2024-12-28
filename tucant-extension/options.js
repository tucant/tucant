/** @type {HTMLInputElement} */
const mobileDesignCheckbox = document.getElementById('mobile-design')
mobileDesignCheckbox.addEventListener("change", event => {
    chrome.storage.sync.set(
        { mobileDesign: event.target.checked },
    );
})

const settings = await chrome.storage.sync.get(
    { mobileDesign: false },
);
mobileDesignCheckbox.checked = settings.mobileDesign;
