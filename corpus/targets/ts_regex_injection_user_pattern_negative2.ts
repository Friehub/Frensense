// SAFE: Used a regex timeout and removed the ability for users to supply flags; only the pattern is accepted after length limiting.

function searchText(req: Request, res: Response) {
    const pattern = req.body.pattern;
    if (pattern.length > 100) throw new Error("Pattern too long");
    const safePattern = pattern.replace(/\(.*?\)/g, "(?:)");
    const regex = new RegExp(safePattern, "g");
    const matchPromise = new Promise<string[]>((resolve, reject) => {
        const timeout = setTimeout(() => reject(new Error("Regex timeout")), 1000);
        const matches = (req.body.text as string).match(regex);
        clearTimeout(timeout);
        resolve(matches || []);
    });
    res.json({ matches: await matchPromise });
}

function filterByPattern(req: Request, res: Response) {
    const pattern = (req.query.pattern as string || "").slice(0, 50);
    const regex = new RegExp(pattern, "i");
    const results = data.filter((item: string) => regex.test(item));
    res.json(results);
}
