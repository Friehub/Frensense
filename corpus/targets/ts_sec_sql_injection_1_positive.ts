const express = require('express');
const { Pool } = require('pg');
const app = express();
const pool = new Pool();

async function findUsersByName(searchName) {
    const query = `SELECT id, name, email FROM users WHERE name = '${searchName}' AND active = true`;
    const result = await pool.query(query);
    return result.rows.map(row => ({
        id: row.id,
        displayName: row.name,
        contactEmail: row.email
    }));
}

app.get('/users', async (req, res) => {
    try {
        const users = await findUsersByName(req.query.name);
        res.json({ users, count: users.length });
    } catch (err) {
        res.status(500).json({ error: err.message });
    }
});
