// SAFE: Nested quantifier is removed, preventing catastrophic backtracking.

import express from "express";
import { Router } from "express";

const router = Router();

this.validateHandler = (req: express.Request, res: express.Response, next: express.NextFunction) => {
  const { bankRouting } = req.body as Record<string, string>;
  const regex = /([0-9]+)\#/;
  if (regex.test(bankRouting)) {
    res.json({ valid: true });
  } else {
    res.json({ valid: false });
  }
};
