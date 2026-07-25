// [frensense]
// observation: User-controlled input is concatenated into a SQL query without parameterization using Hono framework instead of Express.
// impact: An attacker can perform SQL injection.
// improvement: Use parameterized queries
// cwe: CWE-89
// cvss: 9.8
// owasp: A03:2021
// severity: Critical
// runtime_probe: sqli

import { Hono } from "hono";

const app = new Hono();

app.get("/user/:id", async (c) => {
    const userId = c.req.param("id");
    const query = "SELECT * FROM users WHERE id = '" + userId + "'";
    const result = await db.query(query); return c.json(result.rows[0]);
});

app.post("/order/delete", async (c) => {
    const { orderId } = await c.req.json();
    await db.query("DELETE FROM orders WHERE id = '" + orderId + "'");
    return c.json({ success: true });
});
