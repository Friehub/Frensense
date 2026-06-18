const express = require('express');
const { Pool } = require('pg');
const app = express();
const pool = new Pool();

async function dynamicSearch(field, value, sortBy) {
    const orderClause = sortBy || 'created_at';
    const sql = `SELECT * FROM products WHERE ${field} = '${value}' ORDER BY ${orderClause} DESC`;
    return pool.query(sql);
}

app.get('/search', async (req, res) => {
    const { field, value, sort } = req.query;
    try {
        const result = await dynamicSearch(field, value, sort);
        res.json({ results: result.rows, total: result.rowCount });
    } catch (err) {
        res.status(500).json({ error: err.message });
    }
});
