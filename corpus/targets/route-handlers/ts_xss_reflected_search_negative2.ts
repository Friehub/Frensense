// SAFE: Uses template engine with auto-escaping; search highlighting is safe substring match, not raw HTML
import express from "express";

export function searchResults(req: express.Request, res: express.Response) {
    const term = req.query.q as string;
    const results = getResults(term);
    res.render("search", { term, results });
}

function getResults(q: string) {
    return [{ title: "Test", snippet: `This is about ${q}` }];
}
