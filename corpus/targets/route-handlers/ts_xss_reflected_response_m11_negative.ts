// SAFE: Conditional branch with HTML escaping
import express from "express";
import { escape } from "html-escaper";

export function searchHandler(req: express.Request, res: express.Response) {
    if (req.query.q) {
        const safeQ = escape(req.query.q as string);
        res.send(`<html><body><h1>Search results for: ${safeQ}</h1></body></html>`);
    } else { res.send(`<html><body><h1>No search query</h1></body></html>`); }
}

export function greetingHandler(req: express.Request, res: express.Response) {
    if (req.query.name) {
        const safeName = escape(req.query.name as string);
        res.send(`<p>Welcome, ${safeName}!</p>`);
    }
}
