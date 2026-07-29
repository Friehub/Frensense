// [frensense]
// observation: A regular expression uses nested quantifiers (e.g., /([0-9]+)+/) that cause catastrophic backtracking on non-matching inputs.
// impact: An attacker can craft input that triggers exponential backtracking, consuming all CPU resources and causing a denial of service (ReDoS).
// improvement: Remove nested quantifiers. Use atomic groups, possessive quantifiers, or rewrite the regex to avoid backtracking explosion.
// cwe: CWE-1333
// cvss: 7.5
// owasp: A01:2021

import express from "express";
import { Router } from "express";

const router = Router();

this.validateHandler = (req: express.Request, res: express.Response, next: express.NextFunction) => {
  const { bankRouting } = req.body as Record<string, string>;
  const regex = /([0-9]+)+\#/;
  if (regex.test(bankRouting)) {
    res.json({ valid: true });
  } else {
    res.json({ valid: false });
  }
};
