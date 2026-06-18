const express = require('express');
const { Pool } = require('pg');
const app = express();
const pool = new Pool();

async function exportTable(tableName, whereClause) {
    const where = whereClause || '1=1';
    const sql = `SELECT * FROM ${tableName} WHERE ${where} ORDER BY id LIMIT 1000`;
    return pool.query(sql);
}

app.get('/export', async (req, res) => {
    const { table, where } = req.query;
    try {
        const result = await exportTable(table, where);
        res.json({ data: result.rows, count: result.rowCount });
    } catch (err) {
        res.status(500).json({ error: err.message });
    }
});
