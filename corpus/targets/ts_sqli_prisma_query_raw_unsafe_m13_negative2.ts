// SAFE: Drizzle with sql tagged template (alternate)
import { drizzle } from "drizzle-orm/node-postgres";
import { sql } from "drizzle-orm";
const db = drizzle(process.env.DATABASE_URL!);
app.get("/user", async (req: any, res: any) => { const userId = req.query.id; const user = await db.execute(sql`SELECT * FROM users WHERE id = ${userId}`); res.json(user); });
