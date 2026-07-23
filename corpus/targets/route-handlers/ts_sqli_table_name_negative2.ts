// SAFE: Uses a mapping object to translate user-supplied table names to safe values
const TABLE_MAP: Record<string, string> = {
  "products": "products",
  "orders": "orders",
  "users": "users",
};

const COLUMN_MAP: Record<string, string> = {
  "name": "name",
  "createdAt": "created_at",
  "price": "price",
};

async function queryTable(req: Request, res: Response) {
  const table = TABLE_MAP[req.body.tableName];
  if (!table) throw new Error("Invalid table");
  const result = await db.query(`SELECT * FROM ${table}`);
  res.json(result.rows);
}

async function sortResults(req: Request, res: Response) {
  const column = COLUMN_MAP[req.query.sort as string];
  if (!column) throw new Error("Invalid column");
  const rows = await db.query(`SELECT * FROM products ORDER BY ${column}`);
  res.json(rows);
}
