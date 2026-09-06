// SAFE: The raw input is validated for size and structure before being parsed.

import http from "node:http";

const MAX_MESSAGE_SIZE = 8192;

function parseHttpMessage(req: Request, res: Response) {
    const raw = req.body.rawMessage;
    if (typeof raw !== "string" || raw.length > MAX_MESSAGE_SIZE) {
        return res.status(400).json({ error: "Invalid message" });
    }
    if (!raw.startsWith("GET ") && !raw.startsWith("POST ") && !raw.startsWith("PUT ")) {
        return res.status(400).json({ error: "Unsupported method" });
    }
    const parsed = http.parse(raw);
    res.json({
        method: parsed.method,
        url: parsed.url,
        headers: parsed.headers,
    });
}
