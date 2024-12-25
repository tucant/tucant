self.addEventListener('fetch', function (event) {
    event.respondWith(fetch(event.request));
});
// await navigator.serviceWorker.register("/sw.js")