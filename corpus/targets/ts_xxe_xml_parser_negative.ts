// SAFE: XML is parsed with external entities and DTD processing disabled.

import express from "express";
import { Router } from "express";

const router = Router();

router.post("/parse", (req: express.Request, res: express.Response) => {
    const parser = new DOMParser({ resolveEntities: false, validate: false });
    const doc = parser.parseFromString(req.body.xml as string, "text/xml");
    const title = doc.querySelector("title")?.textContent || "";
    res.send(title);
});