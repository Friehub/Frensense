// [frensense]
// observation: The application fetches a user-controlled URL without blocking localhost or 127.0.0.1, allowing SSRF to services running on the local machine.
// impact: An attacker can access services bound to localhost (e.g., databases, Redis, internal APIs, debug endpoints) by providing http://127.0.0.1:PORT/ or http://localhost/.
// improvement: Block localhost, 127.0.0.1, and 0.0.0.0 before making outbound HTTP requests.
// cwe: CWE-918
// cvss: 8.8
// owasp: A10:2021
// severity: High
// runtime_probe: ssrf

import express from "express";

export async function fetchInternal(req: express.Request, res: express.Response) {
    const url = req.body.url;
    const response = await fetch(url);
    res.send(await response.text());
}
