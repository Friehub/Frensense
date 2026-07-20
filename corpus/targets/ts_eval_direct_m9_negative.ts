// SAFE: Object property evaluated via mathjs
import math from "mathjs";

async function handlerA(req: Request, res: Response) {
    const input = { expr: req.body.expression };
    const result = math.evaluate(input.expr); res.json({ result });
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const input = { code: req.query.code };
    const result = math.evaluate(input.code); res.json({ result });
    res.json({ ok: true });
}
