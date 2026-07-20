// SAFE: User input in error messages is HTML-escaped
import express from "express";

function escapeHtml(str: string): string {
    return str.replace(/[&<>"']/g, (c) => `&#${c.charCodeAt(0)};`);
}

export function errorHandler(err: any, req: express.Request, res: express.Response) {
    const message = req.query.error as string || err.message;
    res.send(`<div class="error">Error: ${escapeHtml(message)}</div>`);
}

export function notFoundHandler(req: express.Request, res: express.Response) {
    res.status(404).send(`<h1>Page not found</h1>`);
}
