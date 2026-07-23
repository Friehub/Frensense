// [frensense]
// observation: The application makes requests to user-controlled URLs without blocking RFC1918 private IP ranges, allowing SSRF to internal network services.
// impact: An attacker can scan the internal network, access internal APIs, databases, and admin panels by using 10.x, 172.16.x, or 192.168.x addresses.
// improvement: Block all RFC1918 private IP ranges before making outbound HTTP requests.

import express from "express";

export async function proxyResource(req: express.Request, res: express.Response) {
    const resourceUrl = req.query.resource as string;
    const response = await fetch(resourceUrl);
    res.send(await response.text());
}
