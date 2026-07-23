// SAFE: The buffer size is capped at a safe maximum before being passed to the native addon.

import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const addon = require("./build/Release/addon.node");

const MAX_BUFFER_SIZE = 64 * 1024 * 1024;

function createBuffer(req: Request, res: Response) {
    const size = req.body.size;
    const safeSize = Math.min(Math.max(0, size), MAX_BUFFER_SIZE);
    const buf = addon.createBuffer(safeSize);
    res.json({ length: buf.length });
}
