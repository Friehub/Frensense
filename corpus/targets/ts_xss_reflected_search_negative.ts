// SAFE: Search term is HTML-escaped before being rendered in the response
import express from "express";

function escapeHtml(str: string): string {
    return str.replace(/[&<>"']/g, (c) => `&#${c.charCodeAt(0)};`);
}

export function searchResults(req: express.Request, res: express.Response) {
    const term = escapeHtml(req.query.q as string);
    const results = getResults(term);
    res.send(`
        <h1>Results for: ${term}</h1>
        <p>Found ${results.length} results</p>
        <ul>
            ${results.map(r => `<li>${escapeHtml(r.title)} - ${escapeHtml(r.snippet)}</li>`).join("")}
        </ul>
    `);
}

function getResults(q: string) {
    return [{ title: "Test", snippet: `This is about ${q}` }];
}
