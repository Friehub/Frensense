// SAFE: Reads ownership first, then updates with verification
async function handlerA(req: Request, res: Response, userId: string, db: DB) {
    const id = req.params.id;
    const resource = await db.prepare("SELECT user_id FROM resources WHERE id = ?").bind(id).first();
    if (!resource || resource.user_id !== userId) return res.status(403).json({ error: "Forbidden" });
    await db.prepare("UPDATE resources SET status = ? WHERE id = ?").bind("updated", id).run();
    res.json({ success: true });
}
async function handlerB(req: Request, res: Response, userId: string, db: DB) {
    const itemId = req.body.itemId;
    const item = await db.prepare("SELECT owner_id FROM items WHERE id = ?").bind(itemId).first();
    if (!item || item.owner_id !== userId) return res.status(403).json({ error: "Forbidden" });
    await db.prepare("UPDATE items SET status = ? WHERE id = ?").bind("updated", itemId).run();
    res.json({ success: true });
}
