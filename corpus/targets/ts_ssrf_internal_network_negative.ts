// SAFE: RFC1918 private IP ranges are blocked before making the request
import express from "express";

const PRIVATE_RANGES = [
    /^10\./,
    /^172\.(1[6-9]|2\d|3[01])\./,
    /^192\.168\./,
];

function isSafe(url: string): boolean {
    try {
        const parsed = new URL(url);
        if (parsed.protocol !== "https:") return false;
        const hostname = parsed.hostname;
        if (PRIVATE_RANGES.some(r => r.test(hostname))) return false;
        return true;
    } catch {
        return false;
    }
}

export async function proxyResource(req: express.Request, res: express.Response) {
    const resourceUrl = req.query.resource as string;
    if (!isSafe(resourceUrl)) {
        return res.status(403).json({ error: "Private IPs not allowed" });
    }
    const response = await fetch(resourceUrl, { signal: AbortSignal.timeout(5000) });
    res.send(await response.text());
}
