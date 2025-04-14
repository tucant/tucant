import { handleOpenInTucan, getCurrentTab } from "./open-in-tucan.js"

// TODO try using https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/runtime/sendMessage

// @ts-ignore
window.sayHello = async () => {
    console.log("sending message")
    let response = await chrome.runtime.sendMessage({})
    console.log("sent message")
}