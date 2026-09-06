// SAFE: Conditional branch with encodeURI
import express from "express";
export function searchHandler(req: express.Request, res: express.Response) {
  if (req.query.q) {
    res.send(`<html><body><h1>Search results for: ${encodeURI(req.query.q as string)}</h1></body></html>`);
  }
}
