// [frensense]
// observation: User-controlled input is concatenated into a SQL query without parameterization via a promise .then() chain.
// impact: An attacker can perform SQL injection.
// improvement: Use parameterized queries

function getUserById(req: Request, res: Response) {
    Promise.resolve(req.params.id).then(userId => {
        const query = "SELECT * FROM users WHERE id = '" + userId + "'";
        db.query(query).then(result => res.json(result.rows[0]));
    });
}

function deleteOrder(req: Request, res: Response) {
    new Promise(resolve => resolve(req.body.orderId)).then(orderId => {
        db.query("DELETE FROM orders WHERE id = '" + orderId + "'").then(() => res.json({ success: true }));
    });
}
