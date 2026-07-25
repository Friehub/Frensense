// [frensense]
// observation: User-controlled input is passed directly to the RegExp constructor, allowing injection of regex patterns that can cause ReDoS (catastrophic backtracking).
// impact: An attacker can inject a pattern with nested quantifiers (e.g., (a+)+) that causes catastrophic backtracking, leading to CPU exhaustion and denial of service.
// improvement: Validate or sanitize the regex pattern before constructing the RegExp, or use pattern matching without user-supplied regex.
// cwe: CWE-1333
// cvss: 7.5
// owasp: A06:2021
// severity: High

function searchText(req: Request, res: Response) {
    const pattern = req.body.pattern;
    const text = req.body.text;
    const regex = new RegExp(pattern, req.body.flags || "g");
    const matches = text.match(regex);
    res.json({ matches });
}

function filterByPattern(req: Request, res: Response) {
    const pattern = req.query.pattern as string;
    const flags = req.query.flags as string || "i";
    const regex = new RegExp(pattern, flags);
    const results = data.filter((item: string) => regex.test(item));
    res.json(results);
}
