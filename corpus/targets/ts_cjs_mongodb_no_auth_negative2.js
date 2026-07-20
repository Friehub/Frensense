// SAFE: MongoDB connection uses SCRAM-SHA-256 authentication with credentials loaded from a secure config.

const mongodb = require('mongodb');
const config = require('config');

var url = 'mongodb://' + config.get('db.user') + ':' + encodeURIComponent(config.get('db.password')) + '@' + config.get('db.host') + ':27017/' + config.get('db.name') + '?authMechanism=SCRAM-SHA-256';

mongodb.MongoClient.connect(url, function(err, client) {
  if (err) throw err;
  global.db = client.db(config.get('db.name'));
});
