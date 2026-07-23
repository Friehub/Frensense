// SAFE: Validated header names against an allowlist and removed CRLF characters from header values.

const SAFE_HEADERS = new Set([
    "X-Custom-Header", "X-Request-Id", "X-Trace-Id",
    "Cache-Control", "Content-Language",
]);

function sanitizeHeaderValue(value: string): string {
    return value.replace(/[\r\n]/g, "").replace(/\0/g, "");
}

function handleResponse(req: Request, res: Response) {
    const customHeader = req.body.headerName;
    const customValue = sanitizeHeaderValue(req.body.headerValue);
    if (!SAFE_HEADERS.has(customHeader)) {
        res.status(400).json({ error: "Header not allowed" });
        return;
    }
    res.setHeader(customHeader, customValue);
    res.json({ success: true });
}

function setCorsHeaders(req: Request, res: Response) {
    const origin = req.headers.origin || "";
    const safeOrigin = sanitizeHeaderValue(origin);
    if (safeOrigin === req.headers.origin) {
        res.setHeader("Access-Control-Allow-Origin", safeOrigin);
    }
    res.json({ message: "CORS headers set" });
}
