document.addEventListener('click', function (event) {
    if (event.target.getAttribute("data-bs-hide") !== "collapse") {
        return;
    }
    event.preventDefault()

    if (window.innerWidth >= 1200) {
        return;
    }

    bootstrap.Collapse.getOrCreateInstance(document.querySelector(event.target.getAttribute("data-bs-target")), { toggle: false }).hide()
})
