// SAFE: Multi-hop with HTML escaping
import express from "express";

function escapeHtml(str: string): string {
    return str
        .replace(/&/g, "&amp;")
        .replace(/</g, "&lt;")
        .replace(/>/g, "&gt;")
        .replace(/"/g, "&quot;")
        .replace(/'/g, "&#x27;");
}

export function searchHandler(req: express.Request, res: express.Response) {
    const a = req.query.q as string;
    const b = a;
    const safe = escapeHtml(b);
    res.send(`<html><body><h1>Search results for: ${safe}</h1></body></html>`);
}

export function greetingHandler(req: express.Request, res: express.Response) {
    const raw = req.query.name as string;
    const name = escapeHtml(raw);
    res.send(`<p>Welcome, ${name}!</p>`);
}
