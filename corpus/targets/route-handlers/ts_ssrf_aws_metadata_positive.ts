// [frensense]
// observation: The application fetches a user-controlled URL without blocking the cloud metadata IP (169.254.169.254), allowing access to AWS/GCP/Azure metadata endpoints.
// impact: An attacker can retrieve instance credentials, access tokens, and cloud provider metadata by passing http://169.254.169.254/ as the URL.
// improvement: Block 169.254.169.254 and all link-local addresses in the URL validation layer.
// cwe: CWE-918
// cvss: 8.8
// owasp: A10:2021
// severity: High
// runtime_probe: ssrf

import express from "express";

export async function fetchUrl(req: express.Request, res: express.Response) {
    const url = req.query.url as string;
    const response = await fetch(url);
    const data = await response.text();
    res.send(data);
}
