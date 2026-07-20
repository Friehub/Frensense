// SAFE: DNS resolution is checked against private IP ranges; only public IPs are allowed
import express from "express";
import dns from "dns/promises";

const PRIVATE_RANGES = [/^10\./, /^172\.(1[6-9]|2\d|3[01])\./, /^192\.168\./];

async function isSafeUrl(url: string): Promise<boolean> {
    try {
        const parsed = new URL(url);
        if (parsed.protocol !== "https:") return false;
        const hostname = parsed.hostname;
        if (PRIVATE_RANGES.some(r => r.test(hostname))) return false;
        const addresses = await dns.resolve4(hostname);
        return !addresses.some(addr => PRIVATE_RANGES.some(r => r.test(addr)));
    } catch {
        return false;
    }
}

export async function proxyResource(req: express.Request, res: express.Response) {
    const resourceUrl = req.query.resource as string;
    if (!(await isSafeUrl(resourceUrl))) {
        return res.status(403).json({ error: "Target resolves to private IP" });
    }
    const response = await fetch(resourceUrl, { signal: AbortSignal.timeout(5000) });
    res.send(await response.text());
}
