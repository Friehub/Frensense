// SAFE: The async route handler uses try-catch and forwards errors to next(err), preventing unhandled rejections.

const express = require('express');
const mongodb = require('mongodb');

const app = express();

app.get('/api/users/:id', async function(req, res, next) {
  try {
    var user = await db.collection('users').findOne({ _id: mongodb.ObjectId(req.params.id) });
    res.json(user);
  } catch (err) {
    next(err);
  }
});

app.use(function(err, req, res, next) {
  console.error('Error:', err.message);
  res.status(500).json({ error: 'Internal server error' });
});
