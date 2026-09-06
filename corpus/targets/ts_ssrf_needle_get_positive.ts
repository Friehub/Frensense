// [frensense]
// observation: User-controlled URL is passed directly to needle.get() without host validation, enabling Server-Side Request Forgery (SSRF).
// impact: An attacker can make the server send requests to internal services (localhost, cloud metadata endpoints) or external systems, bypassing firewalls and network ACLs.
// improvement: Validate the URL host against an allowlist of permitted domains. Reject requests to private IP ranges and loopback addresses.
// cwe: CWE-918
// cvss: 8.8
// owasp: A10:2021

import express from "express";
import { Router } from "express";
import needle from "needle";

const router = Router();

this.getResearch = (req: express.Request, res: express.Response, next: express.NextFunction) => {
  const url = (req.query.url as string) + (req.query.symbol as string);
  needle.get(url, (error: any, newResponse: any) => {
    if (!error && newResponse.statusCode === 200) {
      res.send(newResponse.body);
    }
  });
};
