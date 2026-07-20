// SAFE: Renamed variables with JSON.parse
async function handlerA(req: Request, res: Response) {
  const userExpression = req.body.expression;
  try { const result = JSON.parse(userExpression); res.json({ result }); } catch { res.status(400).send("Invalid JSON"); }
  res.json({ ok: true });
}
