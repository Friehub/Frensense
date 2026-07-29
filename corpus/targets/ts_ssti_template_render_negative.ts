// SAFE: Template is a static string compiled at initialization. User input is only passed as data context.

import express from "express";

const app = express();
const compiledTemplate = ejs.compile("<h1><%= title %></h1><p><%= message %></p>");

app.post("/render", (req: express.Request, res: express.Response) => {
    const html = compiledTemplate({ title: req.body.title, message: req.body.body });
    res.send(html);
});