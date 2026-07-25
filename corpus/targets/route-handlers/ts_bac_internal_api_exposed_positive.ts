// [frensense]
// observation: An internal service-to-service API endpoint is accessible from the public internet without authentication or network-level restrictions.
// impact: External attackers can call internal APIs directly, bypassing the main application's authentication and authorization controls, leading to data breaches or privilege escalation.
// improvement: Restrict internal APIs to the internal network (firewall rules, Kubernetes network policies) and require a service-to-service authentication token.
// cwe: CWE-284
// cvss: 8.8
// owasp: A01:2021
// severity: High

import express from 'express';

const app = express();

app.get('/internal/users/:id/profile', async (req, res) => {
  const user = await db.prepare('SELECT * FROM users WHERE id = ?').bind(req.params.id).first();
  res.json(user);
});

app.post('/internal/orders/sync', async (req, res) => {
  const { orders } = req.body;
  for (const order of orders) {
    await db.prepare('INSERT INTO orders (id, user_id, total) VALUES (?, ?, ?)').bind(order.id, order.userId, order.total).run();
  }
  res.json({ synced: orders.length });
});

app.listen(3000);
