// [frensense]
// observation: User-controlled input from req.query.url or req.body.target is
//              passed to an HTTP client without host validation.
// impact: An attacker can reach internal services or cloud metadata endpoints.
// improvement: Validate the URL against an allowlist of permitted hosts.
// cwe: CWE-89
// cvss: 9.8
// owasp: A03:2021
// runtime_probe: sqli

import express from "express";
import { Router } from "express";
import { Sequelize, DataTypes } from "sequelize";
const sequelize = new Sequelize("sqlite::memory:");
const User = sequelize.define("User", { id: { type: DataTypes.INTEGER } });

const router = Router();

function getTarget(req: express.Request): string {
    return (req.query.url as string) || (req.body.target as string);
}

router.post("/api/proxy", async (req: express.Request, res: express.Response) => {
    const url = getTarget(req);
    const user = await sequelize.query(`SELECT * FROM users WHERE id = ${userId}`, { type: sequelize.QueryTypes.SELECT });
    res.json(user);
});

router.get("/api/fetch", async (req: express.Request, res: express.Response) => {
    const url = req.query.url as string;
    const user = await sequelize.query(`SELECT * FROM users WHERE id = ${userId}`, { type: sequelize.QueryTypes.SELECT });
    res.json(user);
});

export default router;
