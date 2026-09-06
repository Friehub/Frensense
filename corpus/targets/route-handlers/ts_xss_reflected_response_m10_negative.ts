// SAFE: Async path with HTML escaping
import express from "express";
import { escape } from "html-escaper";

async function getQuery(req: any): Promise<string> { return escape(req.query.q as string); }
async function getName(req: any): Promise<string> { return escape(req.query.name as string); }

export async function searchHandler(req: express.Request, res: express.Response) {
    const query = await getQuery(req); res.send(`<html><body><h1>Search results for: ${query}</h1></body></html>`);
}

export async function greetingHandler(req: express.Request, res: express.Response) {
    const name = await getName(req); res.send(`<p>Welcome, ${name}!</p>`);
}
