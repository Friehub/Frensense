// SAFE: The native addon uses safe buffer accessors that validate the offset and length before reading or writing.

import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const addon = require("./build/Release/addon_safe.node");

function processBuffer(req: Request, res: Response) {
    const data = req.body.data;
    const buf = Buffer.from(data);
    addon.writeBufferSafe(buf, buf.length);
    res.json({ result: buf.toString() });
}
