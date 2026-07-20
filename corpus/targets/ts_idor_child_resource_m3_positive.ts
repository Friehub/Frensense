// [frensense]
// observation: Child resource ID is accepted from the client without verifying it belongs to the validated parent through multiple variable assignments.
// impact: Attacker supplies a child ID belonging to a different user's parent, enabling cross-account access
// improvement: Verify the child resource's parent matches the validated parent in the same query

async function handlerA(req: Request, res: Response) {
    const a = req.params.childId;
    const b = a;
    const item = await db.prepare("SELECT * FROM child_items WHERE id = ?").bind(b).first(); res.json(item);
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const x = req.body.resourceId;
    const y = x;
    const z = y;
    const item = await db.prepare("SELECT * FROM sub_resources WHERE id = ?").bind(z).first(); res.json(item);
    res.json({ ok: true });
}
