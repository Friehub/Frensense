// SAFE: Async path with encodeURI
import express from "express";
async function sanitizeQuery(req: any): Promise<string> { return encodeURI(req.query.q as string); }
export async function searchHandler(req: express.Request, res: express.Response) {
  const query = await sanitizeQuery(req);
  res.send(`<html><body><h1>Search results for: ${query}</h1></body></html>`);
}
