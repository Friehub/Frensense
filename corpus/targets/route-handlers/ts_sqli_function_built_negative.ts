function buildSearchQuery(field: string, value: string): string {
    const allowedFields = ["name", "email", "status"];
    if (!allowedFields.includes(field)) throw new Error("Invalid field");
    return `SELECT * FROM items WHERE ${field} = $1`;
}

async function searchItems(req: Request, res: Response) {
    const field = req.query.field as string;
    const value = req.query.value as string;
    const query = buildSearchQuery(field, value);
    const results = await db.query(query, [value]);
    res.json(results.rows);
}
