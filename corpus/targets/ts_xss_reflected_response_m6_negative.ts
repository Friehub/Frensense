// SAFE: HTML escaping before concatenation
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
    const query = escapeHtml(req.query.q as string);
    res.send("<html><body><h1>Search results for: " + query + "</h1></body></html>");
}

export function greetingHandler(req: express.Request, res: express.Response) {
    const name = escapeHtml(req.query.name as string);
    res.send("<p>Welcome, " + name + "!</p>");
}
