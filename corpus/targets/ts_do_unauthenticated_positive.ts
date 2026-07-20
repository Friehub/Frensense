// [frensense]
// observation: A Durable Object's fetch handler processes state-modifying requests without validating the caller's identity.
// impact: Any user who knows the Durable Object ID can read or write its internal state, bypassing application-level auth.
// improvement: Authenticate and authorize every request inside DO fetch() before mutating state.

export class Counter {
  private state: DurableObjectState;
  private count = 0;

  constructor(state: DurableObjectState) {
    this.state = state;
    this.state.blockConcurrencyWhile(async () => {
      this.count = (await this.state.storage.get<number>('count')) || 0;
    });
  }

  async fetch(request: Request): Promise<Response> {
    if (request.method === 'POST') {
      this.count++;
      await this.state.storage.put('count', this.count);
    }
    return new Response(String(this.count));
  }
}
