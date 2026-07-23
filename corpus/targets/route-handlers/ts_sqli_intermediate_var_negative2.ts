// SAFE: Escapes single quotes to prevent injection while preserving query structure
function escapeSql(value: string): string {
  return value.replace(/'/g, "''");
}

async function searchProducts(req: Request, res: Response) {
  const input = req.query.category;
  const searchTerm = input;
  const sql = "SELECT * FROM products WHERE category = '" + escapeSql(searchTerm) + "'";
  const results = await db.query(sql);
  res.json(results.rows);
}
