// SAFE: Renamed variables with HTML escaping
import express from "express";
import { escape } from "html-escaper";

export function searchHandler(req: express.Request, res: express.Response) {
    const searchQuery = req.query.q as string;
    const safeQ = escape(searchQuery);
    res.send(`<html><body><h1>Search results for: ${safeQ}</h1></body></html>`);
}

export function greetingHandler(req: express.Request, res: express.Response) {
    const userName = req.query.name as string;
    const safeName = escape(userName);
    res.send(`<p>Welcome, ${safeName}!</p>`);
}
