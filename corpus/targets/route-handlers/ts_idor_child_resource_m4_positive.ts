// [frensense]
// observation: Child resource ID is accepted from the client without verifying it belongs to the validated parent through a helper function.
// impact: Attacker supplies a child ID belonging to a different user's parent, enabling cross-account access
// improvement: Verify the child resource's parent matches the validated parent in the same query

function getValue(input: string): string {
    return input;
}

function prepareInput(raw: any): string {
    return raw;
}

async function handlerA(req: Request, res: Response) {
    const val = getValue(req.params.childId);
    const item = await db.prepare("SELECT * FROM child_items WHERE id = ?").bind(val).first(); res.json(item);
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const val = prepareInput(req.body.resourceId);
    const item = await db.prepare("SELECT * FROM sub_resources WHERE id = ?").bind(val).first(); res.json(item);
    res.json({ ok: true });
}
