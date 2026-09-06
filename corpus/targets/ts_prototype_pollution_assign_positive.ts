// [frensense]
// observation: User-controlled input is merged into an object using Object.assign, allowing prototype pollution via __proto__ or constructor keys.
// impact: An attacker can inject properties into the global Object prototype, bypassing security checks, altering application behavior, or enabling arbitrary code execution.
// improvement: Use a safe merge strategy that filters out __proto__, constructor, and prototype keys. Consider using Object.assign({}, sanitizedInput) with explicit key filtering.
// cwe: CWE-1321
// cvss: 8.2
// owasp: A04:2021

import express from "express";

const app = express();

interface UserConfig {
    name: string;
    role: string;
}

app.post("/update", (req: express.Request, res: express.Response) => {
    const config: UserConfig = { name: "default", role: "user" };
    Object.assign(config, req.body);
    res.json({ status: "updated", config });
});