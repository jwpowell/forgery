import { bootstrap, wasm } from '../engine/bootstrap.js';

bootstrap(function () {
    wasm.run("workspace");

    // TODO: awesome stuff here.
});


