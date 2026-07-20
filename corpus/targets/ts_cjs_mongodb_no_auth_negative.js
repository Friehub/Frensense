// SAFE: The MongoDB connection string includes authentication credentials from environment variables.

const mongodb = require('mongodb');

var url = 'mongodb://' + process.env.MONGO_USER + ':' + process.env.MONGO_PASS + '@localhost:27017/mydb?authSource=admin';

mongodb.MongoClient.connect(url, function(err, client) {
  if (err) throw err;
  console.log('Connected to MongoDB');
  global.db = client.db('mydb');
});
