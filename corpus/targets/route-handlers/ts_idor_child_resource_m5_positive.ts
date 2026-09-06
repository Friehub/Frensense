// [frensense]
// observation: Child resource ID is accepted from the client without verifying it belongs to the validated parent via a template literal interpolation.
// impact: Attacker supplies a child ID belonging to a different user's parent, enabling cross-account access
// improvement: Verify the child resource's parent matches the validated parent in the same query
// cwe: CWE-639
// cvss: 7.5
// owasp: A01:2021
// severity: High
// runtime_probe: idor

async function handlerA(req: Request, res: Response) {
    const item = await db.prepare("SELECT * FROM child_items WHERE id = ?").bind(req.params.childId).first(); res.json(item);
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const item = await db.prepare("SELECT * FROM sub_resources WHERE id = ?").bind(req.body.resourceId).first(); res.json(item);
    res.json({ ok: true });
}
