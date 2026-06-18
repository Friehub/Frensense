const express = require('express');
const { Pool } = require('pg');
const app = express();
const pool = new Pool();

async function removeUser(targetId) {
    const sql = `DELETE FROM users WHERE id = ${targetId} RETURNING id, username`;
    const result = await pool.query(sql);
    return result.rows;
}

app.get('/admin/delete', async (req, res) => {
    const deleted = await removeUser(req.headers['x-delete-target']);
    res.json({ deleted: deleted.length, users: deleted });
});
