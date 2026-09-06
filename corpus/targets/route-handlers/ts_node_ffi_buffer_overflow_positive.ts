// [frensense]
// observation: napi_create_buffer is called with a user-controlled size without validation, potentially causing an OOM condition or integer overflow.
// impact: An attacker can request an extremely large buffer, exhausting system memory (OOM), or cause an integer overflow that creates a small buffer but reports a large size (buffer overflow).
// improvement: Validate the buffer size against a reasonable maximum before calling napi_create_buffer.

import { createRequire } from "node:module";

const require = createRequire(import.meta.url);
const addon = require("./build/Release/addon.node");

function createBuffer(req: Request, res: Response) {
    const size = req.body.size;
    const buf = addon.createBuffer(size);
    res.json({ length: buf.length });
}
