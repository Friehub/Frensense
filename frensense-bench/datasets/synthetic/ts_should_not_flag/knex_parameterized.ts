import knex from "knex";
const db = knex({ client: "pg" });

// Safe: parameterized query — must NOT be flagged (tests bug 2.1)
export async function getUser(req: any, res: any) {
  const id = req.params.id;
  const result = await db("users").where({ id }).select("*");
  res.json(result);
}
