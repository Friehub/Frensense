// SAFE: Renamed variables with encodeURI
import express from "express";
export function searchHandler(req: express.Request, res: express.Response) {
  const searchQuery = req.query.q as string;
  res.send(`<html><body><h1>Search results for: ${encodeURI(searchQuery)}</h1></body></html>`);
}
