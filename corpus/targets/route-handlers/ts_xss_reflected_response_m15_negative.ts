// SAFE: .then() chain with HTML escaping
import express from "express";
import { escape } from "html-escaper";

export function searchHandler(req: express.Request, res: express.Response) {
    Promise.resolve(req.query.q as string).then(query => {
        const safeQ = escape(query);
        res.send(`<html><body><h1>Search results for: ${safeQ}</h1></body></html>`);
    });
}

export function greetingHandler(req: express.Request, res: express.Response) {
    new Promise<string>(resolve => resolve(req.query.name as string)).then(name => {
        const safeName = escape(name);
        res.send(`<p>Welcome, ${safeName}!</p>`);
    });
}
