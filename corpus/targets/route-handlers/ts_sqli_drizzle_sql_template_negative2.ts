// SAFE: Uses Drizzle ORM query builder with parameterized where clause
import { users } from "./schema";
import { eq } from "drizzle-orm";

async function findUser(req: Request, res: Response) {
  const email = req.query.email;
  const result = await db.select().from(users).where(eq(users.email, email));
  res.json(result);
}
