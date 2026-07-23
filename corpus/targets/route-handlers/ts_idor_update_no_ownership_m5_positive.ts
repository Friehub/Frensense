// [frensense]
// observation: UPDATE query uses a user-supplied ID without verifying ownership of the resource via a template literal interpolation.
// impact: An attacker can update another user's resource by supplying their ID
// improvement: Add an ownership check in the WHERE clause: WHERE id = ? AND user_id = ?

async function handlerA(req: Request, res: Response) {
    await db.prepare("UPDATE resources SET status = ? WHERE id = ?").bind("updated", req.params.id).run(); res.json({ success: true });
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    await db.prepare("UPDATE items SET status = ? WHERE id = ?").bind("updated", req.body.itemId).run(); res.json({ success: true });
    res.json({ ok: true });
}
