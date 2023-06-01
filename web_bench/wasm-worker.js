import { threads } from 'wasm-feature-detect';
import * as Comlink from 'comlink';

// Wrap wasm-bindgen exports (the `generate` function) to add time measurement.
function wrapExports({ generate_proof }) {
  return () => {
    const time = generate_proof();
    return time;
  };
}

async function initHandlers() {
  let [singleThread, multiThread] = await Promise.all([
    (async () => {
      const singleThread = await import('./pkg/proof_gen_web_bench.js');
      await singleThread.default();
      return wrapExports(singleThread);
    })(),
    (async () => {
      // If threads are unsupported in this browser, skip this handler.
      if (!(await threads())) return;
      const multiThread = await import(
        './pkg-parallel/proof_gen_web_bench'
      );
      await multiThread.default();
      await multiThread.initThreadPool(navigator.hardwareConcurrency);
      return wrapExports(multiThread);
    })()
  ]);

  return Comlink.proxy({
    singleThread,
    supportsThreads: !!multiThread,
    multiThread
  });
}

Comlink.expose({
  handlers: initHandlers()
});
