// SAFE: Short-lived access token (15 min) combined with refresh token
import jwt from 'jsonwebtoken';

export function issueToken(userId: string): { accessToken: string; refreshToken: string } {
  const accessToken = jwt.sign({ sub: userId, role: 'user' }, process.env.JWT_SECRET!, { expiresIn: '15m' });
  const refreshToken = jwt.sign({ sub: userId, type: 'refresh' }, process.env.JWT_REFRESH_SECRET!, { expiresIn: '7d' });
  return { accessToken, refreshToken };
}
