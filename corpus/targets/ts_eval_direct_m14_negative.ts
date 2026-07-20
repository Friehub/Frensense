// SAFE: Renamed variables with mathjs evaluate
import math from "mathjs";

async function handlerA(req: Request, res: Response) {
    const userExpression = req.body.expression;
    const result = math.evaluate(userExpression); res.json({ result });
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const codeSnippet = req.query.code;
    const result = math.evaluate(codeSnippet); res.json({ result });
    res.json({ ok: true });
}
