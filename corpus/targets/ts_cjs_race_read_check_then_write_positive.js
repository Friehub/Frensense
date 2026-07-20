// [frensense]
// observation: The server reads a value, checks it against a threshold, then writes — all as separate non-atomic operations. Under concurrent requests, this TOCTOU pattern allows bypassing the limit.
// impact: An attacker can send multiple concurrent requests to exceed intended limits (e.g., withdraw more than balance, use a coupon multiple times) before the check sees the updated value.
// improvement: Use MongoDB's findOneAndUpdate with a conditional filter to atomically check and update in a single operation.

const express = require('express');
const mongodb = require('mongodb');

const app = express();
app.use(require('body-parser').json());

app.post('/api/account/withdraw', function(req, res) {
  db.collection('accounts').findOne({ userId: req.session.userId }, function(err, account) {
    if (err) return res.status(500).send(err);
    var amount = parseInt(req.body.amount, 10);
    if (account.balance < amount) {
      return res.status(400).json({ error: 'Insufficient funds' });
    }
    db.collection('accounts').updateOne(
      { userId: req.session.userId },
      { $inc: { balance: -amount } },
      function(err, result) {
        if (err) return res.status(500).send(err);
        res.json({ success: true });
      }
    );
  });
});
