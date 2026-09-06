// [frensense]
// observation: X-Frame-Options or CSP frame-ancestors header is missing.
// impact: Attackers can perform clickjacking — overlay invisible frames.
// improvement: Set X-Frame-Options: DENY or use CSP frame-ancestors.
// cwe: CWE-1021
// cvss: 5.4
// owasp: A05:2021
// runtime_probe: security_headers

import express from "express";
import { Router } from "express";

const router = Router();

router.get("/page", (req: express.Request, res: express.Response) => {
  res.render("template", { user: req.session.user });
});

router.post("/action", (req: express.Request, res: express.Response) => {
  res.json({ status: "ok" });
});

export default router;
