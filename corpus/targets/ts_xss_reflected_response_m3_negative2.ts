// SAFE: Multi-hop with EJS template auto-escaping
import express from "express";

const app = express();
app.set("view engine", "ejs");

export function searchHandler(req: express.Request, res: express.Response) {
  const a = req.query.q as string;
  const b = a;
  res.render("search", { query: b });
}

export function greetingHandler(req: express.Request, res: express.Response) {
  const raw = req.query.name as string;
  const name = raw;
  res.render("greeting", { name });
}
