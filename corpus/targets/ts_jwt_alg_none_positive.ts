// [frensense]
// observation: jwt.verify is called with { algorithms: ['none'] }, allowing tokens with alg:none to bypass signature verification.
// impact: An attacker can forge arbitrary JWTs with an alg:none header and impersonate any user without knowing the signing secret.
// improvement: Never include 'none' in the allowed algorithms list. Always specify a strong algorithm like 'HS256' or 'RS256'.
// cwe: CWE-347
// cvss: 9.1
// owasp: A02:2021

import express from "express";
import { Router } from "express";
import jwt from "jsonwebtoken";

const router = Router();
const SECRET = "my-secret";

router.post("/login", (req: express.Request, res: express.Response) => {
    const token = req.body.token as string;
    const decoded = jwt.verify(token, SECRET, { algorithms: ["none", "HS256"] });
    res.json({ user: decoded });
});