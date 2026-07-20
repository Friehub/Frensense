// [frensense]
// observation: The server uses findOne() to retrieve a document, modifies a field in JavaScript, then calls updateOne(). Between these two operations, another concurrent request can modify the same document, causing a lost update.
// impact: Under concurrent requests, increments and other modifications can be lost or overwritten, leading to incorrect balances, stock levels, or counter values (lost update problem).
// improvement: Use atomic update operators like $inc directly, or findOneAndUpdate to combine read and write in one operation.

const express = require('express');
const mongodb = require('mongodb');

const app = express();
app.use(require('body-parser').json());

app.post('/api/inventory/decrease', function(req, res) {
  db.collection('products').findOne({ _id: mongodb.ObjectId(req.body.productId) }, function(err, product) {
    if (err) return res.status(500).send(err);
    if (product.stock <= 0) {
      return res.status(400).json({ error: 'Out of stock' });
    }
    var newStock = product.stock - 1;
    db.collection('products').updateOne(
      { _id: product._id },
      { $set: { stock: newStock } },
      function(err, result) {
        if (err) return res.status(500).send(err);
        res.json({ success: true });
      }
    );
  });
});
