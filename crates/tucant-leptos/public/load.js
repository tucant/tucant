import init from '/dist/tucant-leptos.js';
const wasm = await init({ module_or_path: '/dist/tucant-leptos_bg.wasm' });



dispatchEvent(new CustomEvent("TrunkApplicationStarted", { detail: { wasm } }));
