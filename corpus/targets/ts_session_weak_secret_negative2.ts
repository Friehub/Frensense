// SAFE: Uses auto-generated secure secret via key rotation helper
import session from 'express-session';
import crypto from 'crypto';

const secret = process.env.SESSION_SECRET || crypto.randomBytes(32).toString('hex');
const app = express();
app.use(session({
  secret: [secret, crypto.randomBytes(32).toString('hex')],
  resave: false,
  saveUninitialized: false,
  name: '__Host-sid',
  cookie: { secure: true, httpOnly: true, sameSite: 'strict' }
}));
