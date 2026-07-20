// SAFE: .then() chain with mathjs evaluate
import math from "mathjs";

function handlerA(req: Request, res: Response) {
    Promise.resolve(req.body.expression).then(val => {
        const result = math.evaluate(val); res.json({ result });
    });
    res.json({ ok: true });
}

function handlerB(req: Request, res: Response) {
    new Promise(resolve => resolve(req.query.code)).then(val => {
        const result = math.evaluate(val); res.json({ result });
    });
    res.json({ ok: true });
}
