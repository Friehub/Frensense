// SAFE: Object property value escaped with encodeURI
import express from "express";

export function searchHandler(req: express.Request, res: express.Response) {
  const input = { q: req.query.q as string };
  res.send(`<html><body><h1>Search results for: ${encodeURI(input.q)}</h1></body></html>`);
}
