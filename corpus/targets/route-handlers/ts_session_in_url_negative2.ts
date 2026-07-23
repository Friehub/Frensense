// SAFE: Session is stored in an HttpOnly secure cookie
export async function authenticate(req: Request): Promise<Response> {
  const token = req.cookies?.session;
  if (!token) return new Response('Unauthorized', { status: 401 });
  const user = await verifyToken(token);
  if (!user) return new Response('Unauthorized', { status: 401 });
  return new Response(JSON.stringify(user));
}

export function setSessionCookie(res: Response, token: string): void {
  res.cookie('session', token, { httpOnly: true, secure: true, sameSite: 'strict', path: '/' });
}
