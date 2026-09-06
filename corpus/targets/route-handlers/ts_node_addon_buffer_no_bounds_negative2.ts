// SAFE: A JavaScript-side validation wrapper ensures the buffer offset and length are within bounds before calling the addon.

import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const addon = require("./build/Release/addon.node");

function writeBufferSafe(buf: Buffer, offset: number, data: Buffer): void {
    if (offset < 0 || offset + data.length > buf.length) {
        throw new RangeError("Buffer write out of bounds");
    }
    addon.writeBuffer(buf, offset, data);
}

function processBuffer(req: Request, res: Response) {
    const data = req.body.data;
    const buf = Buffer.alloc(1024);
    writeBufferSafe(buf, 0, Buffer.from(data));
    res.json({ result: buf.toString() });
}
