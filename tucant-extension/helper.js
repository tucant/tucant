import { handleOpenInTucan, getCurrentTab } from "./open-in-tucan.js"

// @ts-ignore
window.sayHello = async () => {
    console.log("Hello world")
    const id = await chrome.cookies.get({
        url: "https://www.tucan.tu-darmstadt.de/scripts/",
        name: "id",
    })

    let tab = await getCurrentTab()
    console.log(tab)

    if (!tab?.id || !tab.url) {
        console.log("no tab id or url")
        return;
    }

    console.log("opefwewf")
    handleOpenInTucan(id?.value, tab.id, tab.url)
}