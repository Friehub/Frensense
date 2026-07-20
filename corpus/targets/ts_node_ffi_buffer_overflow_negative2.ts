// SAFE: The buffer size is validated against both minimum and maximum bounds before use.

import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const addon = require("./build/Release/addon.node");

const MIN_BUFFER_SIZE = 1;
const MAX_BUFFER_SIZE = 16 * 1024 * 1024;

function createBuffer(req: Request, res: Response) {
    const size = req.body.size;
    if (typeof size !== "number" || !Number.isInteger(size) || size < MIN_BUFFER_SIZE || size > MAX_BUFFER_SIZE) {
        return res.status(400).json({ error: "Invalid buffer size" });
    }
    const buf = addon.createBuffer(size);
    res.json({ length: buf.length });
}
