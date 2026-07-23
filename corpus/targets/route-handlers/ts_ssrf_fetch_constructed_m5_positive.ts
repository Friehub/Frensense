// [frensense]
// observation: User-controlled input is used to construct a URL passed to fetch without validation via a template literal interpolation.
// impact: An attacker can make the server send requests to internal services by controlling URL components
// improvement: Validate URL against an allowlist before fetching

async function handlerA(req: Request, res: Response) {
    const response = await fetch(`https://${req.query.host}/api/data`); const data = await response.json(); res.json(data);
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const response = await fetch(`https://api.example.com/${req.body.path}`); const data = await response.json(); res.json(data);
    res.json({ ok: true });
}
