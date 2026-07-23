// SAFE: Async path with JSON.parse
async function getExpr(req: any): Promise<string> { return req.body.expression; }
async function handlerA(req: Request, res: Response) {
  try { const val = await getExpr(req); const result = JSON.parse(val); res.json({ result }); } catch { res.status(400).send("Invalid JSON"); }
  res.json({ ok: true });
}
