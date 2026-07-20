// SAFE: The callback is unregistered on the native side before the reference is deleted.

import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const addon = require("./build/Release/addon.node");

function setupCallback() {
    const cb = (data: number) => console.log("Callback:", data);
    addon.registerCallback(cb);
    addon.unregisterCallback();
    addon.deleteCallback();
}
