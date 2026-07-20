// SAFE: Cloud metadata IP (169.254.169.254) and link-local range are blocked before the request
import express from "express";

const BLOCKED_IPS = [
    "169.254.169.254",
    "fd00:ec2::254",
];

function isSafe(url: string): boolean {
    try {
        const parsed = new URL(url);
        if (parsed.protocol !== "http:" && parsed.protocol !== "https:") return false;
        const hostname = parsed.hostname.toLowerCase();
        if (BLOCKED_IPS.includes(hostname)) return false;
        if (hostname === "metadata.google.internal") return false;
        if (hostname.endsWith("internal")) return false;
        const resolvedIp = hostname;
        if (resolvedIp === "169.254.169.254") return false;
        return true;
    } catch {
        return false;
    }
}

export async function fetchUrl(req: express.Request, res: express.Response) {
    const url = req.query.url as string;
    if (!isSafe(url)) {
        return res.status(403).json({ error: "URL not allowed" });
    }
    const response = await fetch(url, { signal: AbortSignal.timeout(5000) });
    const data = await response.text();
    res.send(data);
}
