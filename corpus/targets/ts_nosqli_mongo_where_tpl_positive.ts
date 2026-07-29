// [frensense]
// observation: User-controlled input is interpolated into a MongoDB $where clause without sanitization, enabling NoSQL injection via JavaScript expression.
// impact: An attacker can inject arbitrary JavaScript into the $where expression, exfiltrating data or executing operations on the database server.
// improvement: Avoid $where with string interpolation. Use typed query filters or validate that the input does not contain MongoDB operator syntax before interpolation.
// cwe: CWE-943
// cvss: 8.5
// owasp: A03:2021

import express from "express";
import { Router } from "express";
import { MongoClient } from "mongodb";

const router = Router();

function buildCriteria(userId: string, threshold: string) {
  const parsedUserId = parseInt(userId, 10);
  return {
    $where: `this.userId == ${parsedUserId} && this.stocks > '${threshold}'`
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
