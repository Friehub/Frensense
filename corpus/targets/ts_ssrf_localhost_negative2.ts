// SAFE: Resolves the hostname and checks if any resolved IP is loopback before allowing the request
import express from "express";
import dns from "dns/promises";

function isLoopback(ip: string): boolean {
    return ip.startsWith("127.") || ip === "::1" || ip === "0.0.0.0";
}

async function isSafeUrl(url: string): Promise<boolean> {
    try {
        const parsed = new URL(url);
        if (parsed.protocol !== "https:") return false;
        const hostname = parsed.hostname.toLowerCase();
        if (hostname === "localhost" || isLoopback(hostname)) return false;
        const addresses = await dns.resolve4(hostname);
        if (addresses.some(addr => isLoopback(addr))) return false;
        return true;
    } catch {
        return false;
    }
}

export async function fetchInternal(req: express.Request, res: express.Response) {
    const url = req.body.url;
    if (!(await isSafeUrl(url))) {
        return res.status(403).json({ error: "Localhost not allowed" });
    }
    const response = await fetch(url, { signal: AbortSignal.timeout(5000) });
    res.send(await response.text());
}
