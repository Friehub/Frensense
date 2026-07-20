// SAFE: Uses an object mapping to build safe queries without string interpolation
const FIELD_MAP: Record<string, string> = {
  name: "name",
  email: "email",
  status: "status",
};

async function searchItems(req: Request, res: Response) {
  const field = req.query.field as string;
  const value = req.query.value as string;
  const safeField = FIELD_MAP[field];
  if (!safeField) throw new Error("Invalid field");
  const results = await db.query(`SELECT * FROM items WHERE ${safeField} = $1`, [value]);
  res.json(results.rows);
}
