// [frensense]
// observation: The application performs a DNS lookup for a user-provided hostname, then later resolves it again when making the HTTP request. An attacker can exploit this to bypass IP allowlist checks (DNS rebinding).
// impact: The check passes against the first resolution (a safe IP), but the actual fetch resolves to a different IP (e.g., internal metadata IP), enabling SSRF.
// improvement: Resolve the hostname once, validate the IP, and use the resolved IP directly for the request instead of the hostname.
// cwe: CWE-918
// cvss: 8.8
// owasp: A10:2021
// severity: High
// runtime_probe: ssrf

import express from "express";
import dns from "dns/promises";

async function isInternalHost(hostname: string): Promise<boolean> {
    const addresses = await dns.resolve4(hostname);
    for (const addr of addresses) {
        if (addr.startsWith("10.") || addr.startsWith("192.168.") || addr === "127.0.0.1") {
            return true;
        }
    }
    return false;
}

export async function proxyRequest(req: express.Request, res: express.Response) {
    const targetUrl = req.query.url as string;
    const parsed = new URL(targetUrl);
    const isInternal = await isInternalHost(parsed.hostname);
    if (isInternal) {
        return res.status(403).json({ error: "Internal hosts not allowed" });
    }
    const response = await fetch(targetUrl);
    const data = await response.json();
    res.json(data);
}
