// [frensense]
// observation: User-controlled URL is destructured from the request object and passed to fetch() without validation.
// impact: An attacker can make the server send requests to internal services by destructuring a crafted URL.
// improvement: Validate the destructured URL against an allowlist before fetching.
// cwe: CWE-918
// cvss: 8.8
// owasp: A10:2021
// severity: High
// runtime_probe: ssrf

async function fetchUserData(req: Request, res: Response) {
    const { url } = req.query;
    const response = await fetch(url);
    const data = await response.json();
    res.json(data);
}

async function proxyRequest(req: Request, res: Response) {
    const { target } = req.body;
    const result = await fetch(target, {
        method: req.body.method,
        headers: req.body.headers,
    });
    const body = await result.text();
    res.send(body);
}
