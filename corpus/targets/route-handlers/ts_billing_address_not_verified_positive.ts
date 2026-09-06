// [frensense]
// observation: The billing address is stored and used for payment processing without any address verification (AVS check) against the card issuer's records.
// impact: Fraudulent transactions with fake billing addresses are accepted, increasing chargeback risk and regulatory non-compliance.
// improvement: Perform an Address Verification System (AVS) check with the payment processor before accepting the billing address.

import { PrismaClient } from '@prisma/client';
import { Request, Response } from 'express';

const prisma = new PrismaClient();

export async function updateBillingAddress(req: Request, res: Response) {
  const userId = req.user.id;
  const { street, city, state, zip, country } = req.body;

  const user = await prisma.user.update({
    where: { id: userId },
    data: {
      billingStreet: street,
      billingCity: city,
      billingState: state,
      billingZip: zip,
      billingCountry: country,
    },
  });

  res.json({ message: 'Billing address updated' });
}
