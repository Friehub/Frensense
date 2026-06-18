const express = require('express');
const { Pool } = require('pg');
const app = express();
const pool = new Pool();

async function getUserOrders(userId) {
    const sql = `SELECT o.id, o.total, o.created_at FROM orders o WHERE o.user_id = ${userId} AND o.status = 'completed' ORDER BY o.created_at DESC LIMIT 50`;
    const result = await pool.query(sql);
    return result.rows;
}

app.get('/report/:id', async (req, res) => {
    const orders = await getUserOrders(req.params.id);
    res.json({ userId: req.params.id, orders, count: orders.length });
});
