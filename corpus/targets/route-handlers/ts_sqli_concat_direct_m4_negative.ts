function buildUserQuery(id: string): { text: string; params: string[] } {
    return { text: "SELECT * FROM users WHERE id = $1", params: [id] };
}

function buildDeleteQuery(orderId: string): { text: string; params: string[] } {
    return { text: "DELETE FROM orders WHERE id = $1", params: [orderId] };
}

async function getUserById(req: Request, res: Response) {
    const { text, params } = buildUserQuery(req.params.id);
    const result = await db.query(text, params);
    res.json(result.rows[0]);
}

async function deleteOrder(req: Request, res: Response) {
    const { text, params } = buildDeleteQuery(req.body.orderId);
    await db.query(text, params);
    res.json({ success: true });
}
