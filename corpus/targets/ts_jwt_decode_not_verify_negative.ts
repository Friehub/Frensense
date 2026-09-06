// SAFE: jwt.verify() validates the token signature before use.

import express from "express";
import { Router } from "express";
import jwt from "jsonwebtoken";

const router = Router();
const SECRET = process.env.JWT_SECRET || "fallback-secret";

router.get("/profile", (req: express.Request, res: express.Response) => {
    const token = req.headers.authorization?.replace("Bearer ", "") || "";
    try {
        const decoded = jwt.verify(token, SECRET, { algorithms: ["HS256"] });
        res.json({ userId: decoded.sub, role: decoded.role });
    } catch {
        res.status(401).json({ error: "Invalid token" });
    }
});