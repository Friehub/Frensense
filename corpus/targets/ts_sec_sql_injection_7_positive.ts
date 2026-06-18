const express = require('express');
const { Pool } = require('pg');
const app = express();
const pool = new Pool();

async function updateProfile(userId, bio, location) {
    const sql = `UPDATE profiles SET bio = '${bio}', location = '${location}', updated_at = NOW() WHERE user_id = ${userId} RETURNING *`;
    const result = await pool.query(sql);
    return result.rows[0];
}

app.post('/update-profile', async (req, res) => {
    const { bio, location } = req.body;
    const userId = req.headers['x-user-id'];
    const profile = await updateProfile(userId, bio, location);
    res.json({ profile, updated: true });
});
