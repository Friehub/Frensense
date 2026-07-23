// SAFE: Object property value HTML-escaped
import express from "express";

import { escape } from "html-escaper";

export function searchHandler(req: express.Request, res: express.Response) {
    const input = { q: req.query.q as string };
    const safeQuery = escape(input.q);
    res.send(`<html><body><h1>Search results for: ${safeQuery}</h1></body></html>`);
}

export function greetingHandler(req: express.Request, res: express.Response) {
    const info = { name: req.query.name as string };
    const safeName = escape(info.name);
    res.send(`<p>Welcome, ${safeName}!</p>`);
}
