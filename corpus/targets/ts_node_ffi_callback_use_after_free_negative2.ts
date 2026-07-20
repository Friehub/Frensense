// SAFE: A persistent reference is used with a guard flag to prevent invocation after cleanup.

import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const addon = require("./build/Release/addon.node");

function setupCallback() {
    let active = true;
    const cb = (data: number) => {
        if (!active) return;
        console.log("Callback:", data);
    };
    addon.registerCallback(cb);
    active = false;
    addon.deleteCallback();
}
