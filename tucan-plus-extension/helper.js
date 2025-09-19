import { handleOpenInTucan, getCurrentTab } from "./open-in-tucan.js"
import { asyncClosure } from "./utils.js";

// @ts-expect-error define new property on window
window.sayHello = () => {
    asyncClosure(async () => {
        const id = await chrome.cookies.get({
            url: "https://www.tucan.tu-darmstadt.de/scripts",
            name: "id",
        })

        let tab = await getCurrentTab()

        if (!tab.id) {
            console.log("no tab id")
            return;
        }

        await chrome.tabs.update(tab.id, {
            url: await handleOpenInTucan(id?.value, tab.id, document.location.href)
        })
    });
}