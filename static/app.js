// app.js — WASM loader / glue
// Loads the compiled WebAssembly module and initializes client interactivity.

import init from './pkg/portfolio_wasm.js';

async function run() {
  try {
    await init();
  } catch (e) {
    console.warn('WASM init failed, falling back to static HTML:', e);
  }
}

run();
