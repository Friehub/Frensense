// [frensense]
// observation: UPDATE query uses a user-supplied ID without verifying ownership of the resource via string concatenation.
// impact: An attacker can update another user's resource by supplying their ID
// improvement: Add an ownership check in the WHERE clause: WHERE id = ? AND user_id = ?

async function handlerA(req: Request, res: Response) {
    const sql = "UPDATE resources SET status = 'updated' WHERE id = '" + req.params.id + "'"; await db.prepare(sql).run(); res.json({ success: true });
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const sql = "UPDATE items SET status = 'updated' WHERE id = '" + req.body.itemId + "'"; await db.prepare(sql).run(); res.json({ success: true });
    res.json({ ok: true });
}
