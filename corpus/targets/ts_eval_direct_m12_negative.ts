// SAFE: Try-catch with mathjs evaluate
import math from "mathjs";

async function handlerA(req: Request, res: Response) {
    try { const result = math.evaluate(req.body.expression); res.json({ result }); } catch (err) { console.error(err); res.status(500).json({ error: err.message }); }
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    try { const result = math.evaluate(req.query.code); res.json({ result }); } catch (err) { console.error(err); res.status(500).json({ error: err.message }); }
    res.json({ ok: true });
}
