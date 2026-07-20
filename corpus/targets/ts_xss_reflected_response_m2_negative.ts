// SAFE: User input is HTML-escaped before being embedded in the response
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
    const query = req.query.q as string;
    const safe = escapeHtml(query);
    res.send(`<html><body><h1>Search results for: ${safe}</h1></body></html>`);
}

export function greetingHandler(req: express.Request, res: express.Response) {
    const name = escapeHtml(req.query.name as string);
    res.send(`<p>Welcome, ${name}!</p>`);
}
