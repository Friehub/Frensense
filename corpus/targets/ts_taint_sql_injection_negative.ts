function searchUsers(req: Request) {
    const query = req.body.search;
    db.query("SELECT * FROM users WHERE name = $1", [query]);
}
