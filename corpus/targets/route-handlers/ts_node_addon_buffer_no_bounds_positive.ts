// [frensense]
// observation: A native addon returns a Buffer or writes to a Buffer without performing bounds checking, allowing out-of-bounds memory access.
// impact: An attacker can read or write beyond the allocated buffer boundaries, leading to memory corruption, information disclosure, or RCE.
// improvement: Ensure the native addon validates all offsets and lengths before accessing the Buffer, or use safe wrapper functions.

import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const addon = require("./build/Release/addon.node");

function processBuffer(req: Request, res: Response) {
    const data = req.body.data;
    const buf = Buffer.from(data);
    addon.writeBuffer(buf, buf.length);
    res.json({ result: buf.toString() });
}
