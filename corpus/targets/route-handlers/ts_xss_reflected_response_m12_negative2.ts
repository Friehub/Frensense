// SAFE: Try-catch with encodeURI
import express from "express";
export function searchHandler(req: express.Request, res: express.Response) {
  try { res.send(`<html><body><h1>Search results for: ${encodeURI(req.query.q as string)}</h1></body></html>`); } catch (err) { res.status(500).send("Error"); }
}
