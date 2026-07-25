// [frensense]
// observation: A Next.js API route handler (pages/api) performs data operations without any authentication.
// impact: Any client can call the API route and access or modify protected data.
// improvement: Add authentication middleware or inline auth checks to every API route that handles sensitive data.
// cwe: CWE-287
// cvss: 9.8
// owasp: A07:2021
// severity: Critical

import type { NextApiRequest, NextApiResponse } from 'next';
import prisma from '@/lib/prisma';

export default async function handler(req: NextApiRequest, res: NextApiResponse) {
  const { userId } = req.query;
  const user = await prisma.user.findUnique({ where: { id: userId as string } });
  res.json(user);
}
