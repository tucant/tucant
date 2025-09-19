/**
 * 
 * @param {() => Promise<void>} closure 
 */
export function asyncClosure(closure) {
    closure().catch(/** @param error {unknown} */ error => {
        console.error(error)
        chrome.notifications.create({
            type: "basic",
            iconUrl: chrome.runtime.getURL("/icon-512.png"),
            title: "TUCaN Plus extension error",
            message: String(error),
        });
    })
}