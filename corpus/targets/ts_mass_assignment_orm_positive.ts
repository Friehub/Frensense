// [frensense]
// observation: The entire req.body is passed directly to an ORM create call, allowing mass assignment of arbitrary model attributes.
// impact: An attacker can set any model field (e.g., role, isAdmin, balance) by including it in the request body, bypassing intended access controls.
// improvement: Use explicit attribute allowlisting (e.g., User.create({ name: req.body.name, email: req.body.email })) or use ORM-level mass assignment protection (e.g., Rails strong parameters, Prisma select).
// cwe: CWE-915
// cvss: 7.5
// owasp: A01:2021

import express from "express";
import { Router } from "express";

const router = Router();

router.post("/users", async (req: express.Request, res: express.Response) => {
    const user = await prisma.user.create({ data: req.body });
    res.json(user);
});