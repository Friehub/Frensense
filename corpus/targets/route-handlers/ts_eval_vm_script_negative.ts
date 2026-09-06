// SAFE: Replaced vm.Script with a predefined template engine that only interpolates data, not code.

function runScript(req: Request, res: Response) {
    const template = req.body.script;
    const data = req.body.data;
    const result = template.replace(/\{\{(\w+)\}\}/g, (_, key) => String(data[key] ?? ""));
    res.json({ result });
}

function compileAndRun(req: Request, res: Response) {
    const code = req.query.code as string;
    const result = code.replace(/\{\{(\w+)\}\}/g, (_, key) => String(req.body[key] ?? ""));
    res.json({ output: result });
}
