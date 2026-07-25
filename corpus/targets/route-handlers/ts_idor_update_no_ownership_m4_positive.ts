// [frensense]
// observation: UPDATE query uses a user-supplied ID without verifying ownership of the resource through a helper function.
// impact: An attacker can update another user's resource by supplying their ID
// improvement: Add an ownership check in the WHERE clause: WHERE id = ? AND user_id = ?
// cwe: CWE-639
// cvss: 7.5
// owasp: A01:2021
// severity: High
// runtime_probe: idor

function getValue(input: string): string {
    return input;
}

function prepareInput(raw: any): string {
    return raw;
}

async function handlerA(req: Request, res: Response) {
    const val = getValue(req.params.id);
    await db.prepare("UPDATE resources SET status = ? WHERE id = ?").bind("updated", val).run(); res.json({ success: true });
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const val = prepareInput(req.body.itemId);
    await db.prepare("UPDATE items SET status = ? WHERE id = ?").bind("updated", val).run(); res.json({ success: true });
    res.json({ ok: true });
}
