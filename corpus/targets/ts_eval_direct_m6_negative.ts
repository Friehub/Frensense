// SAFE: Implements safe alternative
// SAFE: Sanitized input before evaluation using Function constructor
function processExpression(expr: string) {
    const sanitized = expr.replace(/[^0-9+\-*/() ]/g, "");
    return Function(`"use strict"; return (${sanitized})`)();
}
function handlerA(req: Request, res: Response) {
    const result = processExpression(req.body.expression);
    res.json({ result });
}
function handlerB(req: Request, res: Response) {
    const result = processExpression(req.query.code);
    res.json({ result });
}
