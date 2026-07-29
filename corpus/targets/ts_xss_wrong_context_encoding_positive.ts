// [frensense]
// observation: User input is encoded for HTML context but used in a URL context, leaving the application vulnerable to XSS through href attributes.
// impact: An attacker can inject javascript: URLs or other schemes into link elements, executing arbitrary JavaScript when the link is clicked.
// improvement: Use context-appropriate encoding — encodeForURL() for URL contexts, encodeForHTML() for HTML body contexts. Use separate template variables for different output contexts.
// cwe: CWE-79
// cvss: 6.1
// owasp: A03:2021

import express from "express";
import { Router } from "express";

const router = Router();

this.profileHandler = (req: express.Request, res: express.Response, next: express.NextFunction) => {
  const { website } = req.body as Record<string, string>;
  const safeWebsite = ESAPI.encoder().encodeForHTML(website);
  res.render("profile", {
    website: safeWebsite
  });
};
