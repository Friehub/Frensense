// SAFE: .then() chain with JSON.parse
function handlerA(req: Request, res: Response) {
  Promise.resolve(req.body.expression).then(val => {
    try { const result = JSON.parse(val); res.json({ result }); } catch { res.status(400).send("Invalid JSON"); }
  });
  res.json({ ok: true });
}
