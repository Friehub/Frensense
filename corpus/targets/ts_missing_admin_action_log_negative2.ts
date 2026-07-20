// SAFE alternative: middleware-based admin audit
function auditAdminAction(action: string, getTarget?: (req: Request) => Promise<any>) {
  return async (req: Request, res: Response, next: NextFunction) => {
    const before = getTarget ? await getTarget(req).catch(() => null) : null;
    const originalJson = res.json.bind(res);
    res.json = function (body: any) {
      auditLog.write({
        actor: req.user.id,
        action,
        target: req.params.id || req.body.key,
        before,
        after: body,
        ip: req.ip,
      }).catch(console.error);
      return originalJson(body);
    };
    next();
  };
}

app.delete('/api/admin/users/:id', auditAdminAction('admin.user.delete', async req => {
  return db.queryOne('SELECT * FROM users WHERE id = $1', [req.params.id]);
}), async (req, res) => {
  await db.query('DELETE FROM users WHERE id = $1', [req.params.id]);
  res.json({ status: 'deleted' });
});
