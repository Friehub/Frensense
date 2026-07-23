// SAFE: .then() chain with parameterized query
function getUserById(req: Request, res: Response) {
    Promise.resolve(req.params.id).then(userId => {
        db.query("SELECT * FROM users WHERE id = $1", [userId]).then(result => res.json(result.rows[0]));
    });
}

function deleteOrder(req: Request, res: Response) {
    new Promise(resolve => resolve(req.body.orderId)).then(orderId => {
        db.query("DELETE FROM orders WHERE id = $1", [orderId]).then(() => res.json({ success: true }));
    });
}
