// SAFE: Session secret loaded from environment variable with minimum length validation
import session from 'express-session';
import jwt from 'jsonwebtoken';

const SESSION_SECRET = process.env.SESSION_SECRET!;
if (!SESSION_SECRET || SESSION_SECRET.length < 64) {
  throw new Error('SESSION_SECRET must be at least 64 characters');
}

const app = express();
app.use(session({ secret: SESSION_SECRET, resave: false, saveUninitialized: true }));

function signToken(payload: object): string {
  return jwt.sign(payload, SESSION_SECRET);
}
