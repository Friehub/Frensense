// [frensense]
// observation: A sensitive identifier (userId) is taken directly from URL parameters (req.params) instead of the authenticated session, enabling Insecure Direct Object Reference.
// impact: An attacker can tamper with the userId parameter to access or modify another user's data without authorization.
// improvement: Always derive the current user's identity from the authenticated session (req.session.userId) rather than from URL or body parameters.
// cwe: CWE-639
// cvss: 7.5
// owasp: A01:2021

import express from "express";
import { Router } from "express";

const router = Router();

this.getAllocations = (req: express.Request, res: express.Response, next: express.NextFunction) => {
  const { userId } = req.params;
  res.render("allocations", {
    userId
  });
};
