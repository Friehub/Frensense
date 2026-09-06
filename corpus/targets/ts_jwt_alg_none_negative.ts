// SAFE: jwt.verify uses only strong algorithms, excluding 'none'.

import express from "express";
import { Router } from "express";
import jwt from "jsonwebtoken";

const router = Router();
const SECRET = "my-strong-secret-key-here";

router.post("/login", (req: express.Request, res: express.Response) => {
    const token = req.body.token as string;
    const decoded = jwt.verify(token, SECRET, { algorithms: ["HS256"] });
    res.json({ user: decoded });
});