// SAFE: Implements safe alternative
// SAFE: Ownership verified in the UPDATE WHERE clause
async function handlerA(req: Request, res: Response, userId: string, db: DB) {
    const id = req.params.id;
    const result = await db.prepare("UPDATE resources SET status = ? WHERE id = ? AND user_id = ?").bind("updated", id, userId).run();
    if (result.meta.changes === 0) return res.status(404).json({ error: "Not found" });
    res.json({ success: true });
}
async function handlerB(req: Request, res: Response, userId: string, db: DB) {
    const itemId = req.body.itemId;
    const result = await db.prepare("UPDATE items SET status = ? WHERE id = ? AND owner_id = ?").bind("updated", itemId, userId).run();
    if (result.meta.changes === 0) return res.status(404).json({ error: "Not found" });
    res.json({ success: true });
}
