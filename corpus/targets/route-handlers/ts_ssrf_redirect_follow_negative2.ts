// SAFE: Redirect targets are validated before following — only HTTPS to allowed hosts are permitted
import express from "express";

const ALLOWED_DOMAINS = new Set(["api.trusted.com", "data.trusted.com"]);

function isAllowed(url: string): boolean {
    try {
        const parsed = new URL(url);
        return parsed.protocol === "https:" && ALLOWED_DOMAINS.has(parsed.hostname);
    } catch {
        return false;
    }
}

async function safeFetch(url: string): Promise<Response> {
    const resp = await fetch(url, { redirect: "manual" });
    if (resp.status >= 300 && resp.status < 400) {
        const location = resp.headers.get("location");
        if (!location || !isAllowed(location)) {
            throw new Error("Redirect to disallowed host");
        }
        return fetch(location);
    }
    return resp;
}

export async function fetchExternalData(req: express.Request, res: express.Response) {
    const targetUrl = req.query.url as string;
    if (!isAllowed(targetUrl)) {
        return res.status(403).json({ error: "URL not allowed" });
    }
    const response = await safeFetch(targetUrl);
    const data = await response.json();
    res.json(data);
}
