// [frensense]
// observation: A native callback reference is deleted via napi_delete_reference but the native code continues to invoke the callback, causing a use-after-free.
// impact: Invoking a deleted reference results in undefined behavior, including arbitrary code execution or process crash.
// improvement: Ensure the callback is unregistered on the native side before deleting the reference, or use a persistent reference with proper lifecycle management.

import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const addon = require("./build/Release/addon.node");

function setupCallback() {
    const cb = (data: number) => console.log("Callback:", data);
    addon.registerCallback(cb);
    addon.deleteCallback();
}
