// SAFE: Uses a comprehensive IP blocklist that includes all cloud metadata IPs and well-known internal hostnames
import express from "express";
import { BlockList } from "net";

const blocklist = new BlockList();
blocklist.addAddress("169.254.169.254");
blocklist.addRange("10.0.0.0", "10.255.255.255");
blocklist.addRange("172.16.0.0", "172.31.255.255");
blocklist.addRange("192.168.0.0", "192.168.255.255");
blocklist.addRange("127.0.0.0", "127.255.255.255");
blocklist.addRange("0.0.0.0", "0.255.255.255");

const BLOCKED_HOSTNAMES = new Set([
    "metadata.google.internal",
    "metadata.std.internal",
    "100.100.100.200",
]);

async function isSafe(url: string): Promise<boolean> {
    try {
        const parsed = new URL(url);
        if (parsed.protocol !== "http:" && parsed.protocol !== "https:") return false;
        const hostname = parsed.hostname.toLowerCase();
        if (BLOCKED_HOSTNAMES.has(hostname)) return false;
        const { Resolver } = await import("dns/promises");
        const resolver = new Resolver();
        const addresses = await resolver.resolve4(hostname);
        for (const addr of addresses) {
            if (blocklist.check(addr)) return false;
        }
        return true;
    } catch {
        return false;
    }
}

export async function fetchUrl(req: express.Request, res: express.Response) {
    const url = req.query.url as string;
    if (!(await isSafe(url))) {
        return res.status(403).json({ error: "URL not allowed" });
    }
    const response = await fetch(url, { signal: AbortSignal.timeout(5000) });
    const data = await response.text();
    res.send(data);
}
