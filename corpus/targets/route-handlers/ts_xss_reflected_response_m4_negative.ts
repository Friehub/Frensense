// SAFE: Helper function escapes HTML before returning
import express from "express";

function escapeHtml(str: string): string {
    return str
        .replace(/&/g, "&amp;")
        .replace(/</g, "&lt;")
        .replace(/>/g, "&gt;")
        .replace(/"/g, "&quot;")
        .replace(/'/g, "&#x27;");
}

function getSafeQuery(req: express.Request): string {
    return escapeHtml(req.query.q as string);
}

function getSafeName(req: express.Request): string {
    return escapeHtml(req.query.name as string);
}

export function searchHandler(req: express.Request, res: express.Response) {
    const query = getSafeQuery(req);
    res.send(`<html><body><h1>Search results for: ${query}</h1></body></html>`);
}

export function greetingHandler(req: express.Request, res: express.Response) {
    const name = getSafeName(req);
    res.send(`<p>Welcome, ${name}!</p>`);
}
