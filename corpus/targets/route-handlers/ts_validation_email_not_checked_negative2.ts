// SAFE alternative: express-validator
import { body, validationResult } from 'express-validator';

app.post('/api/register',
  body('email').isEmail().normalizeEmail(),
  body('password').isLength({ min: 8 }),
  async (req, res) => {
    const errors = validationResult(req);
    if (!errors.isEmpty()) return res.status(400).json({ errors: errors.array() });
    const user = await db.createUser({ email: req.body.email, passwordHash: await bcrypt.hash(req.body.password, 12) });
    res.json({ id: user.id });
  }
);
