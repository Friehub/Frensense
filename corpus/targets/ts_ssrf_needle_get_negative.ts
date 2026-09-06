// SAFE: URL is validated against an allowlist before being passed to needle.get().

import express from "express";
import { Router } from "express";
import needle from "needle";

const router = Router();
const ALLOWED_HOSTS = new Set(["api.example.com", "finance.example.com"]);

function validateUrl(raw: string): string | null {
  try {
    const parsed = new URL(raw);
    if (!ALLOWED_HOSTS.has(parsed.hostname)) return null;
    return parsed.toString();
  } catch {
    return null;
  }
}

this.getResearch = (req: express.Request, res: express.Response, next: express.NextFunction) => {
  const raw = (req.query.url as string) + (req.query.symbol as string);
  const url = validateUrl(raw);
  if (!url) {
    return res.status(400).json({ error: "Invalid URL" });
  }
  needle.get(url, (error: any, newResponse: any) => {
    if (!error && newResponse.statusCode === 200) {
      res.send(newResponse.body);
    }
  });
};
