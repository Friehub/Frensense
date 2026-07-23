// SAFE: Conditional branch with mathjs evaluate
import math from "mathjs";

async function handlerA(req: Request, res: Response) {
    if (req.body.expression) {
        const result = math.evaluate(req.body.expression); res.json({ result });
    } else { res.json({ error: "No expression" }); }
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    if (req.query.code && req.query.code.length > 0) {
        const result = math.evaluate(req.query.code); res.json({ result });
    }
    res.json({ ok: true });
}
