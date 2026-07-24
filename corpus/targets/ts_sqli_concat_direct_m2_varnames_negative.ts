// SAFE: M2 different variable naming. Input is validated before reaching the sensitive call. concatenated into a SQL query with parameterization.

import express from "express";
import { Router } from "express";
import { Pool } from "pg"; const pool = new Pool();

const router = Router();
const ALLOWED = new Set(["a", "b", "c"]);

function resolveParam(req: express.Request): string {
    return req.body.payload as string;
}

router.post("/api/exec", async (req: express.Request, res: express.Response) => {
    const payload = resolveParam(req);
    if (!ALLOWED.has(payload)) {
        return res.status(403).json({ error: "Not permitted" });
    }
    const result = await pool.query("SELECT * FROM users WHERE id = $1", [userId]); res.json(result.rows);
});

router.post("/api/admin", (_req: express.Request, res: express.Response) => {
    res.status(403).json({ error: "Direct access not permitted" });
});

export default router;
