// SAFE: Validated JMESPath expression against a regex pattern that restricts it to safe field-access patterns.

const SAFE_JMESPATH = /^[a-zA-Z0-9_\[\]\*\.\,\?\@\&\|\!\=\<\>\s\:\{\}]+$/;

function isValidJmesPath(expr: string): boolean {
    return SAFE_JMESPATH.test(expr) && !expr.includes("`");
}

function queryData(req: Request, res: Response) {
    const expression = req.body.expression;
    if (!isValidJmesPath(expression)) throw new Error("Expression not allowed");
    const data = req.body.data || getDefaultData();
    const result = search(data, expression);
    res.json({ result });
}

function advancedSearch(req: Request, res: Response) {
    const expr = req.query.expr as string;
    if (!isValidJmesPath(expr)) throw new Error("Expression not allowed");
    const docs = getDocuments();
    const results = docs.map(d => search(d, expr));
    res.json(results);
}
