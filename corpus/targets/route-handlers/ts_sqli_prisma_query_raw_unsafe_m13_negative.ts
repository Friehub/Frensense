// SAFE: Drizzle with sql tagged template
import { drizzle } from "drizzle-orm/node-postgres";
import { sql } from "drizzle-orm";
const db = drizzle(process.env.DATABASE_URL!);
app.get("/user", async (req: any, res: any) => { const userId = req.query.id; const user = await db.execute(sql`SELECT * FROM users WHERE id = ${userId}`); res.json(user); });
app.post("/user/email", async (req: any, res: any) => { const newEmail = req.body.email; await db.execute(sql`UPDATE users SET email = ${newEmail} WHERE id = ${req.userId}`); res.json({ success: true }); });
