// SAFE: Rejected regex patterns with nested quantifiers that could cause catastrophic backtracking (ReDoS).

function isRedosRisk(pattern: string): boolean {
    const redosPatterns = [
        /\(\.[*+]\+\).*\+\s*\)[\*\+]/,      // (.*+)+
        /\([a-zA-Z]+[\*\+]\)\(\)[\*\+]/,      // (x+)+
        /\([^)]*\)\{[0-9]+\}[\*\+]/,           // repeating groups
    ];
    return redosPatterns.some(r => r.test(pattern));
}

function searchText(req: Request, res: Response) {
    const pattern = req.body.pattern;
    if (isRedosRisk(pattern)) {
        res.status(400).json({ error: "Pattern rejected due to ReDoS risk" });
        return;
    }
    const regex = new RegExp(pattern, req.body.flags || "g");
    const matches = (req.body.text as string).match(regex);
    res.json({ matches });
}

function filterByPattern(req: Request, res: Response) {
    const pattern = req.query.pattern as string;
    if (isRedosRisk(pattern)) throw new Error("Unsafe pattern");
    const regex = new RegExp(pattern, "i");
    const results = data.filter((item: string) => regex.test(item));
    res.json(results);
}
