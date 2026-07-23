// [frensense]
// observation: An asynchronous route handler uses async/await but has no try-catch or .catch() handler. If an async error or Promise rejection occurs, Express 4 does not catch it, resulting in an unhandled promise rejection.
// impact: An unhandled rejection can crash the process (Node 15+) or silently swallow errors, leading to undefined server behavior and potential denial of service.
// improvement: Wrap async handlers in try-catch with next(err), or use a wrapper like express-async-errors.

const express = require('express');
const mongodb = require('mongodb');

const app = express();

async function handleGetUser(req, res) {
  var user = await db.collection('users').findOne({ _id: mongodb.ObjectId(req.params.id) });
  res.json(user);
}

app.get('/api/users/:id', handleGetUser);
