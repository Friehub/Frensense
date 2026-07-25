// SAFE: Template autoescaping is enabled.

import express from "express";

const app = express();

app.set("view engine", "ejs");

app.get("/user/:name", (req: express.Request, res: express.Response) => {
  res.render("profile", { username: req.params.name });
});

export default app;
