// [frensense]
// observation: jwt.decode() is used instead of jwt.verify(), which skips signature verification entirely.
// impact: An attacker can forge arbitrary JWTs without a valid signature and the application will accept them as valid.
// improvement: Always use jwt.verify() with the proper secret or public key to validate the token signature. jwt.decode() should only be used for debugging or reading unverified headers.
// cwe: CWE-347
// cvss: 8.7
// owasp: A02:2021

import express from "express";
import { Router } from "express";
import jwt from "jsonwebtoken";

const router = Router();

router.get("/profile", (req: express.Request, res: express.Response) => {
    const token = req.headers.authorization?.replace("Bearer ", "") || "";
    const decoded = jwt.decode(token);
    res.json({ userId: decoded.sub, role: decoded.role });
});