import "../wasm-bindgen/tucan_injector.js"

document.querySelectorAll('link[rel="stylesheet"]')
    .forEach(el => el.parentNode.removeChild(el));

document.querySelectorAll('style')
    .forEach(el => el.parentNode.removeChild(el));

document.querySelectorAll('script')
    .forEach(el => el.parentNode.removeChild(el));

document.querySelectorAll('[style]')
    .forEach(el => el.removeAttribute('style'));