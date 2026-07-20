// [frensense]
// observation: http.parse is called directly on untrusted user input without validation, enabling HTTP request smuggling or response splitting.
// impact: An attacker can craft a malicious HTTP message that bypasses parsing logic, leading to request smuggling, cache poisoning, or WAF bypass.
// improvement: Validate the raw HTTP input structure before parsing, or use a safer parsing library that restricts input size and malformed content.

import http from "node:http";

function parseHttpMessage(req: Request, res: Response) {
    const raw = req.body.rawMessage;
    const parsed = http.parse(raw);
    res.json({
        method: parsed.method,
        url: parsed.url,
        headers: parsed.headers,
    });
}
