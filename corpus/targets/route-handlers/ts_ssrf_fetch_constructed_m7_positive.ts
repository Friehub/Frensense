// [frensense]
// observation: User-controlled input is used to construct a URL passed to fetch without validation via destructured object property.
// impact: An attacker can make the server send requests to internal services by controlling URL components
// improvement: Validate URL against an allowlist before fetching
// cwe: CWE-918
// cvss: 8.8
// owasp: A10:2021
// severity: High
// runtime_probe: ssrf

async function handlerA(req: Request, res: Response) {
    const { input } = req.query;
    const url = `https://${input}/api/data`; const response = await fetch(url); const data = await response.json(); res.json(data);
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const { value } = req.body;
    const url = `https://api.example.com/${value}`; const response = await fetch(url); const data = await response.json(); res.json(data);
    res.json({ ok: true });
}
