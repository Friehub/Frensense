// SAFE: Uses a middleware pattern that enforces MFA for specific route groups
function requireMfa(): express.Handler {
  return (req, res, next) => {
    if (req.session.mfaVerified) return next();
    if (req.session.mfaRequired && !req.session.mfaVerified) {
      return res.status(403).json({ error: 'mfa_required', message: 'MFA verification required' });
    }
    next();
  };
}

const router = express.Router();
router.use('/api/sensitive', requireMfa());
router.post('/api/sensitive/transfer', requireMfa(), async (req, res) => {
  const { toAccount, amount } = req.body;
  await db.prepare('UPDATE accounts SET balance = balance - ? WHERE user_id = ?').bind(amount, req.session.userId).run();
  await db.prepare('UPDATE accounts SET balance = balance + ? WHERE id = ?').bind(amount, toAccount).run();
  res.json({ success: true });
});
