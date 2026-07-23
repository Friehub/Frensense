// [frensense]
// observation: UPDATE query uses a user-supplied ID without verifying ownership of the resource through multiple variable assignments.
// impact: An attacker can update another user's resource by supplying their ID
// improvement: Add an ownership check in the WHERE clause: WHERE id = ? AND user_id = ?

async function handlerA(req: Request, res: Response) {
    const a = req.params.id;
    const b = a;
    await db.prepare("UPDATE resources SET status = ? WHERE id = ?").bind("updated", b).run(); res.json({ success: true });
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const x = req.body.itemId;
    const y = x;
    const z = y;
    await db.prepare("UPDATE items SET status = ? WHERE id = ?").bind("updated", z).run(); res.json({ success: true });
    res.json({ ok: true });
}
