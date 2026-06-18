const express = require('express');
const { Pool } = require('pg');
const app = express();
const pool = new Pool();

async function bulkRemove(itemIds) {
    const idList = itemIds.join(',');
    const sql = `DELETE FROM items WHERE id IN (${idList}) RETURNING id, name`;
    const result = await pool.query(sql);
    return result.rows;
}

app.post('/bulk-delete', async (req, res) => {
    const deleted = await bulkRemove(req.body.ids);
    res.json({ deleted: deleted.length, items: deleted });
});
