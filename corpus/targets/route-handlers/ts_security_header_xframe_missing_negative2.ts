// SAFE: X-Frame-Options and CSP frame-ancestors are set.

import express from "express";
import { Router } from "express";

const router = Router();

router.get("/page", (req: express.Request, res: express.Response) => {
  res.setHeader("X-Frame-Options", "SAMEORIGIN");
  res.render("template", { user: req.session.user });
});

router.post("/action", (req: express.Request, res: express.Response) => {
  res.setHeader("Content-Security-Policy", "frame-ancestors 'self'");
  res.json({ status: "ok" });
});

export default router;
