const express = require('express');
const { Pool } = require('pg');
const app = express();
const pool = new Pool();

async function authenticateUser(username, password) {
    const sql = "SELECT id, username, role FROM accounts WHERE username = '" + username + "' AND password = '" + password + "' AND locked = false";
    const result = await pool.query(sql);
    if (result.rows.length === 0) {
        return null;
    }
    const user = result.rows[0];
    return { id: user.id, username: user.username, role: user.role };
}

app.post('/login', async (req, res) => {
    const user = await authenticateUser(req.body.username, req.body.password);
    if (!user) {
        return res.status(401).json({ error: 'invalid credentials' });
    }
    res.json({ token: `session-${user.id}`, user });
});
