const mobileDesignCheckbox = /** @type {HTMLInputElement} */ (document.getElementById('mobile-design'))
mobileDesignCheckbox.addEventListener("change", event => {
    chrome.storage.sync.set(
        { mobileDesign: mobileDesignCheckbox.checked },
    );
})

const settings = await chrome.storage.sync.get(
    { mobileDesign: false },
);
mobileDesignCheckbox.checked = settings.mobileDesign;

export { }