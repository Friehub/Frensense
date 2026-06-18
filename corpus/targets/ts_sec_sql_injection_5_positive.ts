const express = require('express');
const { Pool } = require('pg');
const app = express();
const pool = new Pool();

async function queryLogs(level, source, limit) {
    const sql = `SELECT timestamp, message, metadata FROM logs WHERE level = '${level}' AND source = '${source}' ORDER BY timestamp DESC LIMIT ${limit || 100}`;
    return pool.query(sql);
}

app.get('/logs', async (req, res) => {
    const { level, source, limit } = req.query;
    try {
        const result = await queryLogs(level, source, parseInt(limit));
        res.json({ logs: result.rows, count: result.rowCount });
    } catch (err) {
        res.status(500).json({ error: err.message });
    }
});
