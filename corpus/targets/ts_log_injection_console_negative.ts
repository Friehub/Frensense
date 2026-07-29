// SAFE: User input is sanitized with CRLF character removal before logging.

import express from "express";
import { Router } from "express";

const router = Router();

function sanitizeLogInput(input: string): string {
  return input.replace(/[\r\n]/g, "_");
}

this.loginHandler = (req: express.Request, res: express.Response, next: express.NextFunction) => {
  const { userName } = req.body as Record<string, string>;
  console.log("Error: attempt to login with invalid user: %s", sanitizeLogInput(userName));
  res.json({ error: "Invalid username or password" });
};
