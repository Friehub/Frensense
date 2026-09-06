// SAFE: User input is encoded for the correct context (URL) when used in a link element.

import express from "express";
import { Router } from "express";

const router = Router();

this.profileHandler = (req: express.Request, res: express.Response, next: express.NextFunction) => {
  const { website } = req.body as Record<string, string>;
  const safeWebsiteForUrl = ESAPI.encoder().encodeForURL(website);
  const safeWebsiteForHtml = ESAPI.encoder().encodeForHTML(website);
  res.render("profile", {
    website_url: safeWebsiteForUrl,
    website_html: safeWebsiteForHtml
  });
};
