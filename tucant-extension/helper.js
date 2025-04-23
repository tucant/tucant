import { handleOpenInTucan, getCurrentTab } from "./open-in-tucan.js"

// @ts-ignore
window.sayHello = async () => {
    const id = await chrome.cookies.get({
        url: "https://www.tucan.tu-darmstadt.de/scripts/",
        name: "id",
    })

    let tab = await getCurrentTab()

    if (!tab.id) {
        console.log("no tab id")
        return;
    }

    console.log("opefwewf")
    handleOpenInTucan(id?.value, tab.id, document.location.href)
}