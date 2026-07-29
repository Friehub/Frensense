// SAFE: Only explicitly allowlisted fields are passed to the ORM create call.

import express from "express";
import { Router } from "express";

const router = Router();

router.post("/users", async (req: express.Request, res: express.Response) => {
    const user = await prisma.user.create({
        data: {
            name: req.body.name,
            email: req.body.email,
        },
    });
    res.json(user);
});