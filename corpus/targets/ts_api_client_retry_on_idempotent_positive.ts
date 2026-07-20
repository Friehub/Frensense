// [frensense]
// observation: An API client retries failed requests on non-idempotent HTTP methods (POST, PATCH, DELETE) without an idempotency key.
// impact: Retrying a POST request without idempotency guarantees can create duplicate resources (e.g., duplicate orders, double charges, duplicate user registrations). This causes data integrity issues and financial loss.
// improvement: Only retry idempotent methods (GET, HEAD, PUT) unless an idempotency key mechanism is in place.

async function sendPayment(amount: number): Promise<Response> {
  const response = await fetch('https://api.payments.com/charge', {
    method: 'POST',
    body: JSON.stringify({ amount }),
  });
  if (!response.ok && response.status >= 500) {
    return sendPayment(amount);
  }
  return response;
}
