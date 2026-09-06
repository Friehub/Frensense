// SAFE: DNS is resolved once, the IP is validated, and the request is made directly to the IP, preventing rebinding
import express from "express";
import dns from "dns/promises";
import http from "http";
import https from "https";

const BLOCKED_RANGES = [/^10\./, /^192\.168\./, /^172\.(1[6-9]|2\d|3[01])\./, /^127\./, /^0\./];

function isBlockedIP(ip: string): boolean {
    return BLOCKED_RANGES.some(r => r.test(ip));
}

async function resolveAndFetch(urlStr: string): Promise<Response> {
    const parsed = new URL(urlStr);
    const addresses = await dns.resolve4(parsed.hostname);
    for (const addr of addresses) {
        if (isBlockedIP(addr)) {
            throw new Error("Blocked IP resolved from hostname");
        }
    }
    const ip = addresses[0];
    parsed.hostname = ip;
    return fetch(parsed.toString());
}

export async function proxyRequest(req: express.Request, res: express.Response) {
    const targetUrl = req.query.url as string;
    try {
        const response = await resolveAndFetch(targetUrl);
        const data = await response.json();
        res.json(data);
    } catch (err: any) {
        res.status(403).json({ error: err.message });
    }
}
