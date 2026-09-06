// SAFE: Redirect following is explicitly disabled; only the initial URL is fetched
import express from "express";

export async function fetchExternalData(req: express.Request, res: express.Response) {
    const targetUrl = req.query.url as string;
    const response = await fetch(targetUrl, { redirect: "manual" });
    if (response.status >= 300 && response.status < 400) {
        return res.status(403).json({ error: "Redirects not allowed" });
    }
    const data = await response.json();
    res.json(data);
}

export async function importFromUrl(req: express.Request, res: express.Response) {
    const url = req.body.sourceUrl;
    const resp = await fetch(url, { redirect: "manual" });
    if (resp.status >= 300 && resp.status < 400) {
        return res.status(403).json({ error: "Redirects not allowed" });
    }
    res.send(await resp.text());
}
