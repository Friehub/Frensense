// [frensense]
// observation: User-controlled input is interpolated into raw SQL via $queryRawUnsafe using Drizzle ORM instead of Prisma.
// impact: An attacker can perform SQL injection.
// improvement: Use $queryRaw tagged template literals

import { drizzle } from "drizzle-orm/node-postgres";
import { sql } from "drizzle-orm";

const db = drizzle(process.env.DATABASE_URL!);

app.get("/user", async (req: any, res: any) => { const userId = req.query.id; const user = await db.execute(sql.raw(`SELECT * FROM users WHERE id = '${userId}'`)); res.json(user); });

app.post("/user/email", async (req: any, res: any) => { const newEmail = req.body.email; await db.execute(sql.raw(`UPDATE users SET email = '${newEmail}' WHERE id = ${req.userId}`)); res.json({ success: true }); });
