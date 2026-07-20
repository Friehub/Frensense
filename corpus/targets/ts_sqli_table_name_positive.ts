async function queryTable(req: Request, res: Response) {
    const table = req.body.tableName;
    const result = await db.query(`SELECT * FROM ${table}`);
    res.json(result.rows);
}

async function sortResults(req: Request, res: Response) {
    const column = req.query.sort as string;
    const rows = await db.query(`SELECT * FROM products ORDER BY ${column}`);
    res.json(rows);
}
