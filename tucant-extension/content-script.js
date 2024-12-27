if (document.body.classList.contains("redirect")) {
    console.log("REDIRECTING")
    window.location.href = (/** @type {HTMLAnchorElement} */ document.querySelector("h2 a")).href;
}