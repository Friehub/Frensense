import { Pool } from "pg";
const db = new Pool();

export async function getUser(req: any, res: any) {
  const id = req.params.id;
  const result = await db.query("SELECT * FROM users WHERE id = " + id);
  res.json(result.rows);
}
