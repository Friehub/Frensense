// [frensense]
// observation: User-controlled input is passed to res.setHeader() for header name or value, allowing arbitrary HTTP response header injection.
// impact: An attacker can inject arbitrary HTTP headers into the response, enabling cache poisoning, session fixation, or cross-site scripting via injected headers.
// improvement: Validate header names against an allowlist and sanitize header values to remove CRLF characters.

function handleResponse(req: Request, res: Response) {
    const customHeader = req.body.headerName;
    const customValue = req.body.headerValue;
    res.setHeader(customHeader, customValue);
    res.json({ success: true });
}

function setCorsHeaders(req: Request, res: Response) {
    const origin = req.headers.origin || "";
    res.setHeader("Access-Control-Allow-Origin", origin);
    res.setHeader("Access-Control-Allow-Headers", req.headers["access-control-request-headers"] || "");
    res.json({ message: "CORS headers set" });
}
