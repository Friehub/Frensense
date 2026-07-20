// [frensense]
// observation: User-controlled input is used to construct a URL passed to fetch without validation through an intermediate variable.
// impact: An attacker can make the server send requests to internal services by controlling URL components
// improvement: Validate URL against an allowlist before fetching

async function handlerA(req: Request, res: Response) {
    const val = req.query.host;
    const url = `https://${val}/api/data`; const response = await fetch(url); const data = await response.json(); res.json(data);
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const val = req.body.path;
    const url = `https://api.example.com/${val}`; const response = await fetch(url); const data = await response.json(); res.json(data);
    res.json({ ok: true });
}
