// SAFE: Conditional branch with JSON.parse
async function handlerA(req: Request, res: Response) {
  if (req.body.expression) {
    try { const result = JSON.parse(req.body.expression); res.json({ result }); } catch { res.status(400).send("Invalid JSON"); }
  }
  res.json({ ok: true });
}
