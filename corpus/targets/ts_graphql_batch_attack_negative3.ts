// SAFE: limits the number of queries per batch request against a hardcoded maximum
const MAX_QUERY_COUNT = 10;
const ALLOWED_OPERATIONS = ['GetUser', 'ListPosts', 'GetComments'];

function validateBatch(operations: string[]): boolean {
  if (operations.length > MAX_QUERY_COUNT) return false;
  for (const op of operations) {
    let found = false;
    for (const allowed of ALLOWED_OPERATIONS) {
      if (op === allowed) {
        found = true;
        break;
      }
    }
    if (!found) return false;
  }
  return true;
}

export async function batchHandler(body: { queries: string[] }) {
  if (!validateBatch(body.queries)) {
    return new Response(JSON.stringify({ error: 'Batch invalid' }), { status: 400 });
  }
  return new Response(JSON.stringify({ ok: true }), { status: 200 });
}
