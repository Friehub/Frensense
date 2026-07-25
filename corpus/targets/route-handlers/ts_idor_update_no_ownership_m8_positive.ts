// [frensense]
// observation: UPDATE query uses a user-supplied ID without verifying ownership of the resource via an array element access.
// impact: An attacker can update another user's resource by supplying their ID
// improvement: Add an ownership check in the WHERE clause: WHERE id = ? AND user_id = ?
// cwe: CWE-639
// cvss: 7.5
// owasp: A01:2021
// severity: High
// runtime_probe: idor

async function handlerA(req: Request, res: Response) {
    const arr = [req.params.id];
    await db.prepare("UPDATE resources SET status = ? WHERE id = ?").bind("updated", arr[0]).run(); res.json({ success: true });
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const items = [req.body.itemId];
    await db.prepare("UPDATE items SET status = ? WHERE id = ?").bind("updated", items[0]).run(); res.json({ success: true });
    res.json({ ok: true });
}
