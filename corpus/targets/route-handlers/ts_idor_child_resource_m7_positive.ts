// [frensense]
// observation: Child resource ID is accepted from the client without verifying it belongs to the validated parent via destructured object property.
// impact: Attacker supplies a child ID belonging to a different user's parent, enabling cross-account access
// improvement: Verify the child resource's parent matches the validated parent in the same query
// cwe: CWE-639
// cvss: 7.5
// owasp: A01:2021
// severity: High
// runtime_probe: idor

async function handlerA(req: Request, res: Response) {
    const { input } = req.query;
    const item = await db.prepare("SELECT * FROM child_items WHERE id = ?").bind(input).first(); res.json(item);
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const { value } = req.body;
    const item = await db.prepare("SELECT * FROM sub_resources WHERE id = ?").bind(value).first(); res.json(item);
    res.json({ ok: true });
}
