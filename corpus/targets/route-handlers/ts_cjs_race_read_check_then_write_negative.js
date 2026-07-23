// SAFE: The check and update are combined atomically using findOneAndUpdate with a conditional filter.

const express = require('express');
const mongodb = require('mongodb');

const app = express();
app.use(require('body-parser').json());

app.post('/api/account/withdraw', function(req, res) {
  var amount = parseInt(req.body.amount, 10);
  db.collection('accounts').findOneAndUpdate(
    { userId: req.session.userId, balance: { $gte: amount } },
    { $inc: { balance: -amount } },
    { returnDocument: 'after' },
    function(err, result) {
      if (err) return res.status(500).send(err);
      if (!result.value) {
        return res.status(400).json({ error: 'Insufficient funds' });
      }
      res.json({ success: true, newBalance: result.value.balance });
    }
  );
});
