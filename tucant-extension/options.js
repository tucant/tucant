import { asyncClosure } from "./utils";

const mobileDesignCheckbox = /** @type {HTMLInputElement} */ (document.getElementById('mobile-design'))
mobileDesignCheckbox.addEventListener("change", () => {
    asyncClosure(async () => {
        await chrome.storage.sync.set(
            { mobileDesign: mobileDesignCheckbox.checked },
        );
    });
})

const customUICheckbox = /** @type {HTMLInputElement} */ (document.getElementById('custom-ui'))
customUICheckbox.addEventListener("change", () => {
    asyncClosure(async () => {
        await chrome.storage.sync.set(
            { customUi: customUICheckbox.checked },
        );
    })
})


const fixSessionIdInUrlCheckbox = /** @type {HTMLInputElement} */ (document.getElementById('fix-session-id-in-url'))
fixSessionIdInUrlCheckbox.addEventListener("change", () => {
    asyncClosure(async () => {
        await chrome.storage.sync.set(
            { fixSessionIdInUrl: fixSessionIdInUrlCheckbox.checked },
        );
    })
})

/** @type {{ mobileDesign: boolean; customUi: boolean; fixSessionIdInUrl: boolean }} */
const settings = await chrome.storage.sync.get(
    { mobileDesign: false, customUi: true, fixSessionIdInUrl: true },
);
mobileDesignCheckbox.checked = settings.mobileDesign;
customUICheckbox.checked = settings.customUi;
fixSessionIdInUrlCheckbox.checked = settings.fixSessionIdInUrl;

export { }