// [frensense]
// observation: A string method (.trim()) is called on a value from req.body without validating its type, enabling denial of service via HTTP Parameter Pollution (HPP) that passes an array instead of a string.
// impact: An attacker can send multiple values for the same parameter (e.g., firstName=foo&firstName=bar), causing Express to return an array, and calling .trim() on an array throws a TypeError, crashing the request handler.
// improvement: Validate the type of user input before calling string-specific methods. Convert arrays to strings explicitly or reject non-string input.
// cwe: CWE-754
// cvss: 5.3
// owasp: A01:2021

import express from "express";
import { Router } from "express";

const router = Router();

this.profileHandler = (req: express.Request, res: express.Response, next: express.NextFunction) => {
  const { firstName } = req.body as Record<string, string>;
  const trimmed = firstName.trim();
  res.json({ firstName: trimmed });
};
