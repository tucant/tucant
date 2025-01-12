// if you activate the extension while having a tucan page open we want to find out the session id in any way possible

const imprintInFooter = /** @type {HTMLAnchorElement} */ (document.getElementById("pageFootControl_imp"))

if (document.body.classList.contains("access_denied")) {
    document.cookie = `id=; Secure; expires=Thu, 01 Jan 1970 00:00:00 UTC`;
} else if (document.body.classList.contains("redirect")) {
    const sessionId = document.getElementById("sessionId")
    if (sessionId?.innerText === "000000000000001") {
        document.cookie = `id=; Secure; expires=Thu, 01 Jan 1970 00:00:00 UTC`;
    } else {
        document.cookie = `id=${sessionId?.innerText}; Secure`;
    }
} else if (location.href === "https://www.tucan.tu-darmstadt.de/scripts/mgrqispi.dll") {
} else if (imprintInFooter && document.cookie.includes("cnsc=")) {
    const args = /** @type {string} */ (new URL(imprintInFooter.href).searchParams.get("ARGUMENTS"))
    const sessionId = /^-N(?<id>\d+),/.exec(args)?.groups?.id
    if (sessionId === "000000000000001") {
        document.cookie = `id=; Secure; expires=Thu, 01 Jan 1970 00:00:00 UTC`;
    } else {
        document.cookie = `id=${sessionId}; Secure`;
    }
} else {
    console.log("unknown part")
}