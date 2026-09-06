// SAFE: Uses a single DNS resolution and caches the result; the fetch uses the resolved IP with Host header set to original hostname
import express from "express";
import dns from "dns/promises";

const DNS_CACHE = new Map<string, string[]>();
const BLOCKED_RANGES = [/^10\./, /^192\.168\./, /^172\.(1[6-9]|2\d|3[01])\./, /^127\./, /^0\./];

async function resolveHost(hostname: string): Promise<string> {
    if (DNS_CACHE.has(hostname)) {
        return DNS_CACHE.get(hostname)![0];
    }
    const addresses = await dns.resolve4(hostname);
    DNS_CACHE.set(hostname, addresses);
    return addresses[0];
}

async function safeFetch(urlStr: string): Promise<Response> {
    const parsed = new URL(urlStr);
    const ip = await resolveHost(parsed.hostname);
    if (BLOCKED_RANGES.some(r => r.test(ip))) {
        throw new Error(`Blocked IP: ${ip}`);
    }
    const fetchUrl = urlStr.replace(parsed.hostname, ip);
    return fetch(fetchUrl, { headers: { Host: parsed.hostname } });
}

export async function proxyRequest(req: express.Request, res: express.Response) {
    try {
        const response = await safeFetch(req.query.url as string);
        const data = await response.json();
        res.json(data);
    } catch (err: any) {
        res.status(403).json({ error: err.message });
    }
}
