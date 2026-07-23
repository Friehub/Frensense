// SAFE: .then() chain with encodeURI
import express from "express";
export function searchHandler(req: express.Request, res: express.Response) {
  Promise.resolve(req.query.q as string).then(query => {
    res.send(`<html><body><h1>Search results for: ${encodeURI(query)}</h1></body></html>`);
  });
}
