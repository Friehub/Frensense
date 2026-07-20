// SAFE: Localhost and loopback addresses are blocked before making the request
import express from "express";

const BLOCKED_HOSTS = new Set([
    "localhost",
    "127.0.0.1",
    "0.0.0.0",
    "::1",
    "127.0.1.1",
]);

function isSafe(url: string): boolean {
    try {
        const parsed = new URL(url);
        if (parsed.protocol !== "https:") return false;
        const hostname = parsed.hostname.toLowerCase();
        if (BLOCKED_HOSTS.has(hostname)) return false;
        if (hostname.startsWith("127.")) return false;
        return true;
    } catch {
        return false;
    }
}

export async function fetchInternal(req: express.Request, res: express.Response) {
    const url = req.body.url;
    if (!isSafe(url)) {
        return res.status(403).json({ error: "Localhost not allowed" });
    }
    const response = await fetch(url, { signal: AbortSignal.timeout(5000) });
    res.send(await response.text());
}
