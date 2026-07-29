// [frensense]
// observation: User-controlled input is passed to console.log without sanitization, enabling log injection (CRLF injection) attacks.
// impact: An attacker can forge log entries by injecting newlines or special characters, corrupting log analysis, evading detection, or exploiting log viewers.
// improvement: Encode or sanitize user input before logging. Use structured logging libraries that handle special characters, or remove newline characters from log output.
// cwe: CWE-117
// cvss: 5.3
// owasp: A09:2021

import express from "express";
import { Router } from "express";

const router = Router();

this.loginHandler = (req: express.Request, res: express.Response, next: express.NextFunction) => {
  const { userName } = req.body as Record<string, string>;
  console.log("Error: attempt to login with invalid user: ", userName);
  res.json({ error: "Invalid username or password" });
};
