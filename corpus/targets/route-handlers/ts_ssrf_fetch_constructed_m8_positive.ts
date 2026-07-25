// [frensense]
// observation: User-controlled input is used to construct a URL passed to fetch without validation via an array element access.
// impact: An attacker can make the server send requests to internal services by controlling URL components
// improvement: Validate URL against an allowlist before fetching
// cwe: CWE-918
// cvss: 8.8
// owasp: A10:2021
// severity: High
// runtime_probe: ssrf

async function handlerA(req: Request, res: Response) {
    const arr = [req.query.host];
    const response = await fetch(`https://${arr[0]}/api/data`); const data = await response.json(); res.json(data);
    res.json({ ok: true });
}

async function handlerB(req: Request, res: Response) {
    const items = [req.body.path];
    const response = await fetch(`https://api.example.com/${items[0]}`); const data = await response.json(); res.json(data);
    res.json({ ok: true });
}
