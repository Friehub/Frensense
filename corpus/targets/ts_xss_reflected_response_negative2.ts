// SAFE: Template engine with auto-escaping prevents injection
import express from "express";

const app = express();
app.set("view engine", "ejs");

export function searchHandler(req: express.Request, res: express.Response) {
    const query = req.query.q as string;
    res.render("search", { query });
}

export function greetingHandler(req: express.Request, res: express.Response) {
    const name = req.query.name as string;
    res.render("greeting", { name });
}
