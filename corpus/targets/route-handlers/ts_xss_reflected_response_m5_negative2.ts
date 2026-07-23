// SAFE: Template engine auto-escaping instead of raw template literal
import express from "express";

const app = express();
app.set("view engine", "ejs");

export function searchHandler(req: express.Request, res: express.Response) {
  const query = req.query.q as string;
  res.render("search", { query });
}

export function greetingHandler(req: express.Request, res: express.Response) {
  res.render("greeting", { name: req.query.name });
}
