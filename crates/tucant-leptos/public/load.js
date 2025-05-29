import init from './tucant-leptos.js';
const wasm = await init({ module_or_path: './tucant-leptos_bg.wasm' });



dispatchEvent(new CustomEvent("TrunkApplicationStarted", { detail: { wasm } }));
