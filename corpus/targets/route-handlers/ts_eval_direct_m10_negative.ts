// SAFE: Async path with mathjs evaluate
import math from "mathjs";

async function getExpr(req: any): Promise<string> { return req.body.expression; }
async function getCode(req: any): Promise<string> { return req.query.code; }

async function handlerA(req: Request, res: Response) {
    const val = await getExpr(req); const result = math.evaluate(val); res.json({ result });
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const val = await getCode(req); const result = math.evaluate(val); res.json({ result });
    res.json({ ok: true });
}
