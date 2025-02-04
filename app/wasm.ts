import mod from 'logicx/logicx_bg.wasm';
// @ts-ignore
import * as util from 'logicx/logicx_bg.js';

const wasm = new WebAssembly.Instance(new WebAssembly.Module(mod), {
    "./logicx_bg.js": util
});

util.__wbg_set_wasm(wasm.exports);

// @ts-ignore
export * from 'logicx/logicx_bg.js';