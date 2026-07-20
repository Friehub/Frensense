// SAFE: Durable Object validates a bearer token before allowing state mutations

export class Counter {
  private state: DurableObjectState;
  private count = 0;
  private validTokens: string[];

  constructor(state: DurableObjectState) {
    this.state = state;
    this.validTokens = [state.env?.ADMIN_TOKEN || 'default-token'];
  }

  async fetch(request: Request): Promise<Response> {
    const auth = request.headers.get('Authorization');
    if (!auth || !this.validTokens.includes(auth.replace('Bearer ', ''))) {
      return new Response('Unauthorized', { status: 401 });
    }
    if (request.method === 'POST') {
      this.count++;
      await this.state.storage.put('count', this.count);
    }
    return new Response(String(this.count));
  }
}
