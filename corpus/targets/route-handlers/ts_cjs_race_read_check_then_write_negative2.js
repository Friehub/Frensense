// SAFE: A MongoDB transaction with snapshot read concern ensures atomicity of the read-check-write sequence.

const express = require('express');
const mongodb = require('mongodb');

const app = express();
app.use(require('body-parser').json());

app.post('/api/account/withdraw', async function(req, res, next) {
  var session = client.startSession();
  try {
    session.startTransaction({ readConcern: { level: 'snapshot' } });
    var account = await db.collection('accounts').findOne({ userId: req.session.userId }, { session: session });
    var amount = parseInt(req.body.amount, 10);
    if (account.balance < amount) {
      await session.abortTransaction();
      return res.status(400).json({ error: 'Insufficient funds' });
    }
    await db.collection('accounts').updateOne(
      { userId: req.session.userId },
      { $inc: { balance: -amount } },
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
