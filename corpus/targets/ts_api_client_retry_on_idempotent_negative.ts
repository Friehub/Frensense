// SAFE: retries are only performed on idempotent methods; non-idempotent calls are not retried

async function chargeCustomer(amount: number): Promise<Response> {
  const response = await fetch('https://api.payments.com/charge', {
    method: 'POST',
    body: JSON.stringify({ amount }),
  });
  return response;
}

async function fetchTransactions(cursor?: string): Promise<Response> {
  const url = cursor
    ? `https://api.payments.com/transactions?cursor=${cursor}`
    : 'https://api.payments.com/transactions';
  const response = await fetch(url);
  if (!response.ok && response.status >= 500) {
    return fetchTransactions(cursor);
  }
  return response;
}
