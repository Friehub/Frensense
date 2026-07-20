// SAFE alternative: Router-level auth middleware for all admin routes

var express = require('express');
var adminRouter = express.Router();

function isAuthenticated(req, res, next) {
  if (!req.session.userId) {
    return res.status(401).json({ error: 'Authentication required' });
  }
  next();
}

adminRouter.use(isAuthenticated);

adminRouter.get('/users', function(req, res) {
  db.collection('users').find({}).toArray(function(err, users) {
    if (err) return res.status(500).json({ error: 'Server error' });
    res.json(users);
  });
});

adminRouter.delete('/users/:id', function(req, res) {
  db.collection('users').deleteOne({ _id: req.params.id }, function(err, result) {
    if (err) return res.status(500).json({ error: 'Delete failed' });
    res.json({ success: true });
  });
});

module.exports = function(app, db) {
  app.use('/admin', adminRouter);
};
