// [frensense]
// observation: User-controlled input is directly interpolated into the HTML response without escaping across an async/await boundary.
// impact: An attacker can inject arbitrary HTML/JavaScript (XSS).
// improvement: Encode all user input before embedding in HTML output

import express from "express";

async function getQuery(req: any): Promise<string> { return req.query.q as string; }
async function getName(req: any): Promise<string> { return req.query.name as string; }

export async function searchHandler(req: express.Request, res: express.Response) {
    const query = await getQuery(req);
    res.send(`<html><body><h1>Search results for: ${query}</h1></body></html>`);
}

export async function greetingHandler(req: express.Request, res: express.Response) {
    const name = await getName(req);
    res.send(`<p>Welcome, ${name}!</p>`);
}
