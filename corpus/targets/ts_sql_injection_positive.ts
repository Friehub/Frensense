// Rule: TS_SQL_INJECTION
function getUser(req: any, res: any) {
    const id = req.query.id;
    db.query(id);
}
