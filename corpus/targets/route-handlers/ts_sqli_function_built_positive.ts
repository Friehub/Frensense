function buildSearchQuery(field: string, value: string): string {
    return `SELECT * FROM items WHERE ${field} = '${value}'`;
}

async function searchItems(req: Request, res: Response) {
    const field = req.query.field as string;
    const value = req.query.value as string;
    const query = buildSearchQuery(field, value);
    const results = await db.query(query);
    res.json(results.rows);
}
