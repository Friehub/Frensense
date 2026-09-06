// SAFE: User input is passed as a bind parameter instead of being interpolated into the SQL string.

import express from "express";
import { Router } from "express";
import { Pool } from "pg"; const pool = new Pool();

const router = Router();

router.post("/api/login", (req: express.Request, res: express.Response, next: express.NextFunction) => {
    const email = req.body.email;
    pool.query("SELECT * FROM users WHERE email = $1", [email]).then((result: any) => {
        res.json(result.rows);
    }).catch((err: Error) => {
        next(err);
    });
});

export default router;
