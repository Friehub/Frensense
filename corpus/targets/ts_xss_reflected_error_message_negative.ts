// SAFE: Error messages are generic and do not echo user input
import express from "express";

export function errorHandler(err: any, req: express.Request, res: express.Response) {
    console.error("Error:", err.message);
    res.status(500).send(`<div class="error">An unexpected error occurred. Please try again later.</div>`);
}

export function notFoundHandler(req: express.Request, res: express.Response) {
    res.status(404).send(`<h1>Page not found</h1>`);
}
