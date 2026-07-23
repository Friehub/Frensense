// [frensense]
// observation: After an initial HTTP request, the server follows a redirect to a location specified by the external server without validating the redirect target, allowing SSRF via redirection.
// impact: An attacker hosts a server that redirects to http://169.254.169.254/ (cloud metadata) or internal IPs; the application follows the redirect and discloses internal data.
// improvement: Disable redirect following, or validate the redirect target URL before following it.

import express from "express";

export async function fetchExternalData(req: express.Request, res: express.Response) {
    const targetUrl = req.query.url as string;
    const response = await fetch(targetUrl);
    const data = await response.json();
    res.json(data);
}

export async function importFromUrl(req: express.Request, res: express.Response) {
    const url = req.body.sourceUrl;
    const resp = await fetch(url, { redirect: "follow" });
    res.send(await resp.text());
}
