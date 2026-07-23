// SAFE: Middleware verifies tenant ID from URL matches session
function verifyTenantAccess(req: Request, res: Response, next: NextFunction): void {
  const sessionTenantId = req.session?.tenantId;
  const urlTenantId = req.params.tenantId;
  if (!urlTenantId || urlTenantId !== sessionTenantId) {
    res.status(403).json({ error: 'Forbidden' });
    return;
  }
  next();
}

const router = express.Router();
router.use('/api/tenant/:tenantId', verifyTenantAccess);

router.get('/api/tenant/:tenantId/dashboard', async (req, res) => {
  const data = await db.prepare('SELECT * FROM dashboard WHERE tenant_id = ?').bind(req.params.tenantId).first();
  res.json(data);
});

router.get('/api/tenant/:tenantId/users', async (req, res) => {
  const users = await db.prepare('SELECT * FROM users WHERE tenant_id = ?').bind(req.params.tenantId).all();
  res.json(users);
});
