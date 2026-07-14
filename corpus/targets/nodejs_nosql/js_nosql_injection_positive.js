function getUser(req, res) {
    const username = req.body.username;
    // Positive: NoSQL injection via un-sanitized object input
    db.collection("users").findOne({ username: username }, function(err, user) {
        if (err) return res.status(500).send(err);
        res.json(user);
    });
}

function updateProfile(req, res) {
    const userId = req.params.id;
    const query = req.query.query;
    // Positive: $where injection / un-sanitized query object
    db.collection("profiles").find({ $where: `this.id == ${userId}` }).toArray((err, results) => {
        res.send(results);
    });
}
