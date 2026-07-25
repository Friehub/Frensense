// [frensense]
// observation: The MongoDB connection string does not include authentication credentials, connecting to the database without verifying the client's identity.
// impact: An attacker who discovers the MongoDB port can connect directly to the database without authentication, accessing, modifying, or deleting all data stored in the database.
// improvement: Include username and password in the MongoDB connection string and enable authentication on the MongoDB server.
// cwe: CWE-287
// cvss: 9.8
// owasp: A07:2021
// severity: Critical

const mongodb = require('mongodb');

const url = 'mongodb://localhost:27017/mydb';

mongodb.MongoClient.connect(url, function(err, client) {
  if (err) throw err;
  console.log('Connected to MongoDB');
  global.db = client.db('mydb');
});
