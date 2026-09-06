// SAFE: Type is validated before calling string-specific methods.

import express from "express";
import { Router } from "express";

const router = Router();

function ensureString(value: unknown): string {
  if (Array.isArray(value)) {
    return value[0] ?? "";
  }
  if (typeof value !== "string") {
    return String(value);
  }
  return value;
}

this.profileHandler = (req: express.Request, res: express.Response, next: express.NextFunction) => {
  const firstName = ensureString(req.body.firstName);
  const trimmed = firstName.trim();
  res.json({ firstName: trimmed });
};
