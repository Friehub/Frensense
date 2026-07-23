// SAFE: Try-catch with parameterized query
async function getUserById(req: Request, res: Response) {
    try { const result = await db.query("SELECT * FROM users WHERE id = $1", [req.params.id]); res.json(result.rows[0]); } catch (err) { console.error(err); res.status(500).send("Error"); }
}

async function deleteOrder(req: Request, res: Response) {
    try { await db.query("DELETE FROM orders WHERE id = $1", [req.body.orderId]); res.json({ success: true }); } catch (err) { console.error(err); res.status(500).send("Error"); }
}
