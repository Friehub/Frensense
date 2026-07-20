// SAFE: uses idempotency key header for retries on non-idempotent POST operations

import { randomUUID } from 'node:crypto';

let attemptCount = 0;

async function sendPayment(amount: number, idempotencyKey = randomUUID()): Promise<Response> {
  const response = await fetch('https://api.payments.com/charge', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'Idempotency-Key': idempotencyKey,
    },
    body: JSON.stringify({ amount }),
  });
  if (!response.ok && response.status >= 500 && attemptCount < 3) {
    attemptCount++;
    return sendPayment(amount, idempotencyKey);
  }
  attemptCount = 0;
  return response;
}
