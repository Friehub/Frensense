function getUser(req, res) {
    const username = String(req.body.username);
    // Negative: Input is cast to string, preventing object injection
    db.collection("users").findOne({ username: username }, function(err, user) {
        if (err) return res.status(500).send(err);
        res.json(user);
    });
}

function updateProfile(req, res) {
    const userId = parseInt(req.params.id, 10);
    // Negative: Input is cast to int, preventing $where injection
    db.collection("profiles").find({ id: userId }).toArray((err, results) => {
        res.send(results);
    });
}
