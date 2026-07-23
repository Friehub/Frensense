const ALLOWED_TABLES = ["products", "orders", "users"];
const ALLOWED_COLUMNS = ["name", "created_at", "price"];

async function queryTable(req: Request, res: Response) {
    const table = req.body.tableName;
    if (!ALLOWED_TABLES.includes(table)) throw new Error("Invalid table");
    const result = await db.query(`SELECT * FROM ${table}`);
    res.json(result.rows);
}

async function sortResults(req: Request, res: Response) {
    const column = req.query.sort as string;
    if (!ALLOWED_COLUMNS.includes(column)) throw new Error("Invalid column");
    const rows = await db.query(`SELECT * FROM products ORDER BY ${column}`);
    res.json(rows);
}
