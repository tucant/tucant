/** @type {<T>(selector: string) => T} */
function querySelector(selector) {
    const value = document.querySelector(selector)
    if (value !== null) {
        return /** @type {any} */ (value)
    } else {
        throw new Error()
    }
}