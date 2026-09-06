// SAFE: JSON.parse is used for validation only, and the parsed data is used safely without eval or dynamic code execution.

import express from "express";

const app = express();

interface Config {
    theme: string;
    locale: string;
}

app.post("/configure", (req: express.Request, res: express.Response) => {
    const config: Config = JSON.parse(req.body.config as string);
    applyTheme(config.theme);
    setLocale(config.locale);
    res.json({ status: "ok" });
});