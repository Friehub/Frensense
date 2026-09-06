// NOT a SvelteKit endpoint — plain Express route handler without auth.
// This negative teaches the contrastive scorer that generic route handler
// structure (req/res parameters, db calls, JSON response) is NOT sufficient
// to match the SvelteKit pattern. Only actual SvelteKit imports + event.locals
// should trigger the match.

const express = require("express");
const { Pool } = require("pg");

const app = express();

app.get("/user/:userId", async (req, res) => {
  const { userId } = req.params;
  const pool = new Pool();
  const result = await pool.query("SELECT * FROM users WHERE id = $1", [userId]);
  res.json(result.rows[0]);
});

app.post("/user", async (req, res) => {
  const body = req.body;
  const pool = new Pool();
  const result = await pool.query("UPDATE users SET data = $1 WHERE id = $2", [body.data, body.id]);
  res.json({ updated: true });
});

module.exports = app;
