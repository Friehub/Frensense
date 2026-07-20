import { sql } from "drizzle-orm";

async function findUser(req: Request, res: Response) {
    const email = req.query.email;
    const result = await db.execute(sql.raw(`SELECT * FROM users WHERE email = '${email}'`));
    res.json(result.rows);
}
