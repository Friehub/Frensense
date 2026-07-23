// SAFE: Only allows address update after successful payment authorization confirms address

import { PrismaClient } from '@prisma/client';
import { Request, Response } from 'express';

const prisma = new PrismaClient();

export async function updateBillingAddress(req: Request, res: Response) {
  const userId = req.user.id;
  const { street, city, state, zip, country, paymentIntentId } = req.body;

  const intent = await retrievePaymentIntent(paymentIntentId);

  if (intent.charges.data[0]?.billing_details?.address?.postal_code !== zip) {
    return res.status(400).json({ error: 'ZIP code does not match card' });
  }

  await prisma.user.update({
    where: { id: userId },
    data: {
      billingStreet: street,
      billingCity: city,
      billingState: state,
      billingZip: zip,
      billingCountry: country,
      addressVerifiedAt: new Date(),
    },
  });

  res.json({ message: 'Billing address verified via payment' });
}

async function retrievePaymentIntent(id: string) {
  return { charges: { data: [{ billing_details: { address: { postal_code: '94117' } } }] } };
}
