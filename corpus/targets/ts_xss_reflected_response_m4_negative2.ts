// SAFE: Helper returns value, render uses auto-escaping template
import express from "express";

function getQuery(req: express.Request): string {
  return req.query.q as string;
}

function getName(req: express.Request): string {
  return req.query.name as string;
}

const app = express();
app.set("view engine", "ejs");

export function searchHandler(req: express.Request, res: express.Response) {
  const query = getQuery(req);
  res.render("search", { query });
}

export function greetingHandler(req: express.Request, res: express.Response) {
  const name = getName(req);
  res.render("greeting", { name });
}
