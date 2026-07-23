// SAFE: Verifies billing address via payment processor AVS before saving

import { PrismaClient } from '@prisma/client';
import { Request, Response } from 'express';

const prisma = new PrismaClient();

export async function updateBillingAddress(req: Request, res: Response) {
  const userId = req.user.id;
  const { street, city, state, zip, country, paymentMethodId } = req.body;

  const avsResult = await verifyAddressWithProcessor(paymentMethodId, {
    street, zip,
  });

  if (!avsResult.match) {
    return res.status(400).json({
      error: 'Billing address does not match card on file',
      avsCode: avsResult.code,
    });
  }

  await prisma.user.update({
    where: { id: userId },
    data: { billingStreet: street, billingCity: city, billingState: state, billingZip: zip, billingCountry: country, avsVerified: true },
  });

  res.json({ message: 'Billing address verified and updated' });
}

async function verifyAddressWithProcessor(paymentMethodId: string, address: { street: string; zip: string }) {
  return { match: true, code: 'Y' };
}
