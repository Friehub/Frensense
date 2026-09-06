// SAFE: Used Express res.set() with an object of fixed headers, never accepting header names from the user.

function handleResponse(req: Request, res: Response) {
    res.set({
        "X-Request-Id": req.headers["x-request-id"] || crypto.randomUUID(),
        "Content-Type": "application/json",
    });
    res.json({ success: true });
}

function setCorsHeaders(req: Request, res: Response) {
    const allowedOrigins = ["https://example.com", "https://app.example.com"];
    const origin = req.headers.origin || "";
    if (allowedOrigins.includes(origin)) {
        res.setHeader("Access-Control-Allow-Origin", origin);
    }
    res.json({ message: "CORS headers set" });
}
