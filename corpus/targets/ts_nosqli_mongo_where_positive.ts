// [frensense]
// observation: User-controlled input is passed directly as a MongoDB query filter containing $where, enabling NoSQL injection.
// impact: An attacker can inject MongoDB operators ($where, $regex, $gt, $ne) to bypass authentication, extract data, or execute arbitrary JavaScript on the database server.
// improvement: Validate and sanitize query inputs. Reject or escape MongoDB operators. Use typed query builders or allowlist-based filtering instead of passing raw user objects to query methods.
// cwe: CWE-943
// cvss: 8.5
// owasp: A03:2021

import express from "express";
import { Router } from "express";

const router = Router();

router.post("/find", async (req: express.Request, res: express.Response) => {
    const filter = req.body.filter;
    const results = await db.collection("users").find(filter).toArray();
    res.json(results);
});