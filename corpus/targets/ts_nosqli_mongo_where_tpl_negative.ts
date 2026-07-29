// SAFE: User input is validated and coverted to number before $where interpolation, preventing injection.

import express from "express";
import { Router } from "express";
import { MongoClient } from "mongodb";

const router = Router();

function sanitizeThreshold(raw: string): number {
  const parsed = parseInt(raw, 10);
  if (isNaN(parsed) || parsed < 0 || parsed > 99) {
    throw new Error("Invalid threshold");
  }
  return parsed;
}

function buildCriteria(userId: string, threshold: string) {
  const parsedUserId = parseInt(userId, 10);
  const safeThreshold = sanitizeThreshold(threshold);
  return {
    $where: `this.userId == ${parsedUserId} && this.stocks > ${safeThreshold}`
  };
}

this.getAllocations = (req: express.Request, res: express.Response, next: express.NextFunction) => {
  const { userId, threshold } = req.query as Record<string, string>;
  const criteria = buildCriteria(userId, threshold);
  db.collection("allocations").find(criteria).toArray((err: any, results: any[]) => {
    if (err) return next(err);
    res.json(results);
  });
};
