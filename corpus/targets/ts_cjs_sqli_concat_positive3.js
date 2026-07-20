// [frensense]
// observation: User input is concatenated directly into SQL queries without parameterization.
// impact: An attacker can execute arbitrary SQL commands by crafting input with SQL metacharacters.
// improvement: Use parameterized queries or prepared statements to separate SQL logic from data.

var mysql = require('mysql');
var pool = mysql.createPool({ host: 'localhost', user: 'web', database: 'blog', connectionLimit: 10 });

function searchPosts(req, res) {
  var term = req.query.q;
  var query = `SELECT * FROM posts WHERE title LIKE '%${term}%' OR body LIKE '%${term}%'`;
  pool.query(query, function(err, posts) {
    if (err) return res.status(500).send(err);
    res.json(posts);
  });
}

function updateUserRole(req, res) {
  var role = req.body.role;
  var userId = req.params.id;
  var stmt = `UPDATE users SET role = '${role}' WHERE id = ${userId}`;
  pool.query(stmt, function(err, result) {
    if (err) return res.status(500).send(err);
    res.json({ updated: result.changedRows });
  });
}
