import { Pool } from "pg";
const db = new Pool();

// Safe: SQL in a test file — must NOT be flagged (environment detection)
describe("user queries", () => {
  it("fetches user", async () => {
    const id = "test-id-123";
    const result = await db.query("SELECT * FROM users WHERE id = " + id);
    expect(result.rows).toBeDefined();
  });
});
