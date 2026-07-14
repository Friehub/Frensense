// [frensense]
// observation: An HTTP call made with a URL validated against an allowlist before use.
// impact: None — URL is sanitized and restricted to known-safe hosts.
// improvement: N/A — this is the correct pattern.

import { URL } from "url";

const ALLOWED_HOSTS = new Set(["api.example.com", "cdn.example.com"]);

async function proxyToAllowedHost(targetUrl: string): Promise<Response> {
    const parsed = new URL(targetUrl);
    if (!ALLOWED_HOSTS.has(parsed.hostname)) {
        throw new Error("Host not allowed");
    }
    return fetch(targetUrl);
}

async function fetchFromCdn(path: string): Promise<Buffer> {
    const base = "https://cdn.example.com";
    const response = await fetch(`${base}/${encodeURIComponent(path)}`);
    const buf = await response.arrayBuffer();
    return Buffer.from(buf);
}
