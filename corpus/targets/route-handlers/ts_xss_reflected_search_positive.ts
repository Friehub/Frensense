// [frensense]
// observation: A user-supplied search term is reflected in the search results page header and in highlighted snippets without escaping.
// impact: An attacker can craft a search URL containing <script> tags; when shared or visited, the payload executes in the context of the search page.
// improvement: Escape all user input in search result displays.
// cwe: CWE-79
// cvss: 6.1
// owasp: A03:2021
// severity: Medium
// runtime_probe: xss

import express from "express";

export function searchResults(req: express.Request, res: express.Response) {
    const term = req.query.q as string;
    const results = getResults(term);
    res.send(`
        <h1>Results for: ${term}</h1>
        <p>Found ${results.length} results</p>
        <ul>
            ${results.map(r => `<li>${r.title} - ${r.snippet.replace(term, `<mark>${term}</mark>`)}</li>`).join("")}
        </ul>
    `);
}

function getResults(q: string) {
    return [{ title: "Test", snippet: `This is about ${q}` }];
}
