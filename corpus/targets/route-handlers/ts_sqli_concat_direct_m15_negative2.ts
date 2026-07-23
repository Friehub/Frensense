// SAFE: .then() chain with parameterized query (alternate)
function getUserById(req: Request, res: Response) {
    Promise.resolve(req.params.id).then(userId => {
        db.query("SELECT * FROM users WHERE id = $1", [userId]).then(result => res.json(result.rows[0]));
    });
}
