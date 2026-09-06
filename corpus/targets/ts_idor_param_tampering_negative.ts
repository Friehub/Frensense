// SAFE: User identity is derived from the authenticated session, not from URL parameters.

import express from "express";
import { Router } from "express";

const router = Router();

this.getAllocations = (req: express.Request, res: express.Response, next: express.NextFunction) => {
  const { userId } = req.session as { userId: string };
  res.render("allocations", {
    userId
  });
};
