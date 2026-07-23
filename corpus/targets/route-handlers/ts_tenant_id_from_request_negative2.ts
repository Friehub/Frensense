// SAFE: Tenant ID is embedded in the JWT token and extracted server-side
import jwt from 'jsonwebtoken';

export async function getWorkspaceData(req: Request, db: DB): Promise<Response> {
  const token = req.headers.authorization?.split(' ')[1];
  const payload = jwt.verify(token, process.env.JWT_SECRET!) as any;
  const data = await db.prepare('SELECT * FROM workspace_data WHERE tenant_id = ?').bind(payload.tenantId).all();
  return new Response(JSON.stringify(data));
}
