// SAFE: A transaction ensures the read and write happen in an isolated, atomic sequence.

const express = require('express');
const mongodb = require('mongodb');

const app = express();
app.use(require('body-parser').json());

app.post('/api/inventory/decrease', async function(req, res, next) {
  var session = client.startSession();
  try {
    session.startTransaction();
    var product = await db.collection('products').findOne(
      { _id: mongodb.ObjectId(req.body.productId) },
      { session: session }
    );
    if (!product || product.stock <= 0) {
      await session.abortTransaction();
      return res.status(400).json({ error: 'Out of stock' });
    }
    await db.collection('products').updateOne(
      { _id: product._id },
      { $set: { stock: product.stock - 1 } },
      { session: session }
    );
    await session.commitTransaction();
    res.json({ success: true });
  } catch (err) {
    await session.abortTransaction();
    next(err);
  } finally {
    session.endSession();
  }
});
