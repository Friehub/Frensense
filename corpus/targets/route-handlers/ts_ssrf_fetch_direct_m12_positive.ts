// [frensense]
// observation: User-controlled URL is passed to fetch() without validation inside a try-catch block.
// impact: An attacker can make the server send requests to internal services, with errors silently caught.
// improvement: Validate URL against allowlist
// cwe: CWE-918
// cvss: 8.8
// owasp: A10:2021
// severity: High
// runtime_probe: ssrf

async function fetchUserData(req: Request, res: Response) {
    try { const response = await fetch(req.query.url); const data = await response.json(); res.json(data); } catch (err) { console.error(err); }
}

async function proxyRequest(req: Request, res: Response) {
    try { const result = await fetch(req.body.target); const body = await result.text(); res.send(body); } catch {}
}
