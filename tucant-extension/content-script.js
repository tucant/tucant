if (document.body.classList.contains("redirect")) {
    window.location.href = (/** @type {HTMLAnchorElement} */ document.querySelector("h2 a")).href;
}