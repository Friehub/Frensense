// SAFE: Uses Prisma with salt-rounds precomputed
import { PrismaClient } from '@prisma/client';
import bcrypt from 'bcrypt';

const prisma = new PrismaClient();

export async function signup(req: Request): Promise<Response> {
  const { email, password } = await req.json();
  const hash = await bcrypt.hash(password, 10);
  await prisma.user.create({ data: { email, passwordHash: hash } });
  return new Response('Created', { status: 201 });
}

export async function signin(req: Request): Promise<Response> {
  const { email, password } = await req.json();
  const user = await prisma.user.findUnique({ where: { email } });
  if (!user) return new Response('Unauthorized', { status: 401 });
  const match = await bcrypt.compare(password, user.passwordHash);
  if (!match) return new Response('Unauthorized', { status: 401 });
  const token = jwt.sign({ sub: user.id }, process.env.JWT_SECRET!);
  return new Response(JSON.stringify({ token }));
}
