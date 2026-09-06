// SAFE: Concatenation avoided — uses EJS template auto-escaping
import express from "express";

const app = express();
app.set("view engine", "ejs");

export function searchHandler(req: express.Request, res: express.Response) {
  res.render("search", { query: req.query.q });
}

export function greetingHandler(req: express.Request, res: express.Response) {
  res.render("greeting", { name: req.query.name });
}
