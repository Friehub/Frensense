// [frensense]
// observation: User-controlled input is used to construct a URL passed to fetch without validation through multiple variable assignments.
// impact: An attacker can make the server send requests to internal services by controlling URL components
// improvement: Validate URL against an allowlist before fetching

async function handlerA(req: Request, res: Response) {
    const a = req.query.host;
    const b = a;
    const url = `https://${b}/api/data`; const response = await fetch(url); const data = await response.json(); res.json(data);
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const x = req.body.path;
    const y = x;
    const z = y;
    const url = `https://api.example.com/${z}`; const response = await fetch(url); const data = await response.json(); res.json(data);
    res.json({ ok: true });
}
