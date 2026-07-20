// SAFE: Try-catch with JSON.parse
async function handlerA(req: Request, res: Response) {
  try { const result = JSON.parse(req.body.expression); res.json({ result }); } catch (err) { res.status(400).json({ error: "Invalid JSON" }); }
  res.json({ ok: true });
}
