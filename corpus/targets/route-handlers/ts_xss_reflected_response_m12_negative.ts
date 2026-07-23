// SAFE: Try-catch with HTML escaping
import express from "express";
import { escape } from "html-escaper";

export function searchHandler(req: express.Request, res: express.Response) {
    try { const safeQ = escape(req.query.q as string); res.send(`<html><body><h1>Search results for: ${safeQ}</h1></body></html>`); } catch (err) { console.error(err); res.status(500).send("Error"); }
}

export function greetingHandler(req: express.Request, res: express.Response) {
    try { const safeName = escape(req.query.name as string); res.send(`<p>Welcome, ${safeName}!</p>`); } catch (err) { console.error(err); res.status(500).send("Error"); }
}
