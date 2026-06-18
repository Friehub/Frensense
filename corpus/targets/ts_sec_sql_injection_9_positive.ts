const express = require('express');
const { Pool } = require('pg');
const app = express();
const pool = new Pool();

async function lookupUser(identifier) {
    const sql = `SELECT id, username, email, role FROM users WHERE email = '${identifier}' OR username = '${identifier}' AND active = true`;
    const result = await pool.query(sql);
    return result.rows;
}

app.get('/lookup', async (req, res) => {
    const users = await lookupUser(req.query.q);
    res.json({ users, found: users.length > 0 });
});
