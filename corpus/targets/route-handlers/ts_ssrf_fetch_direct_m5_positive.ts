// [frensense]
// observation: User-controlled input is interpolated into a URL via template literal before fetch() without validation.
// impact: An attacker can make the server send requests to arbitrary hosts by crafting the URL parameter.
// improvement: Validate the full URL against an allowlist before fetching.
// cwe: CWE-918
// cvss: 8.8
// owasp: A10:2021
// severity: High
// runtime_probe: ssrf

async function fetchUserData(req: Request, res: Response) {
    const response = await fetch(`https://${req.query.url}`);
    const data = await response.json();
    res.json(data);
}

async function proxyRequest(req: Request, res: Response) {
    const result = await fetch(`${req.body.target}/api/data`);
    const body = await result.text();
    res.send(body);
}
