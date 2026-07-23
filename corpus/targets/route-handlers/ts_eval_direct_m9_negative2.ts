// SAFE: Object property via JSON.parse
async function handlerA(req: Request, res: Response) {
  const input = { expr: req.body.expression };
  try {
    const result = JSON.parse(input.expr); res.json({ result });
  } catch { res.status(400).send("Invalid JSON"); }
  res.json({ ok: true });
}
