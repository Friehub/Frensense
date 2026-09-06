// SAFE: The stock decrement is done atomically with $inc and a conditional filter that checks stock > 0.

const express = require('express');
const mongodb = require('mongodb');

const app = express();
app.use(require('body-parser').json());

app.post('/api/inventory/decrease', function(req, res) {
  db.collection('products').findOneAndUpdate(
    { _id: mongodb.ObjectId(req.body.productId), stock: { $gt: 0 } },
    { $inc: { stock: -1 } },
    { returnDocument: 'after' },
    function(err, result) {
      if (err) return res.status(500).send(err);
      if (!result.value) {
        return res.status(400).json({ error: 'Out of stock' });
      }
      res.json({ success: true, remaining: result.value.stock });
    }
  );
});
