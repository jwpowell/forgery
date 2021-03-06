// Use ES module import syntax to import functionality from the module
// that we have compiled.
//
// Note that the `default` import is an initialization function which
// will "boot" the module and make it ready to use. Currently browsers
// don't support natively imported WebAssembly as an ES module, but
// eventually the manual initialization won't be required!
import * as wasm from './wasm/engine.js';
export { wasm };

let wasm_initialized = false;
export const bootstrap = function (callback) {
    if (wasm_initialized) {
        callback(wasm);
        return;
    }

    async function init() {
        // First up we need to actually load the wasm file, so we use the
        // default export to inform it where the wasm file is located on the
        // server, and then we wait on the returned promise to wait for the
        // wasm to be loaded.
        //
        // Note that instead of a string here you can also pass in an instance
        // of `WebAssembly.Module` which allows you to compile your own module.
        // Also note that the promise, when resolved, yields the wasm module's
        // exports which is the same as importing the `*_bg` module in other
        // modes    
        await wasm.default('./engine/wasm/engine_bg.wasm');

        wasm_initialized = true;

        // And afterwards we can use all the functionality defined in wasm.
        callback(wasm);
    };
    init();
};

