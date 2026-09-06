// [frensense]
// observation: Template engine has autoescaping disabled.
// impact: User input rendered in templates can execute arbitrary JS.
// improvement: Enable autoescape or use a framework with it on by default.
// cwe: CWE-79
// cvss: 6.1
// owasp: A03:2021
// runtime_probe: xss

import express from "express";

const app = express();
const swig = require("swig");

swig.setDefaults({ autoescape: false });
app.engine("html", swig.renderFile);
app.set("view engine", "html");

app.get("/user/:name", (req: express.Request, res: express.Response) => {
  res.render("profile", { username: req.params.name });
});

export default app;
