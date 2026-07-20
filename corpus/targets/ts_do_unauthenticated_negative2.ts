// SAFE: The DO uses a token introspection endpoint to verify the caller's identity and permissions

export class Counter {
  private state: DurableObjectState;

  constructor(state: DurableObjectState) {
    this.state = state;
  }

  async fetch(request: Request): Promise<Response> {
    const token = request.headers.get('Authorization')?.replace('Bearer ', '');
    if (!token) return new Response('Unauthorized', { status: 401 });

    const introspection = await fetch('https://auth.example.com/introspect', {
      headers: { 'Authorization': `Bearer ${token}` }
    });
    const session = await introspection.json() as { active: boolean, scope: string };
    if (!session.active || !session.scope.includes('counter:write')) {
      return new Response('Forbidden', { status: 403 });
    }

    if (request.method === 'POST') {
      const count = (await this.state.storage.get<number>('count')) || 0;
      await this.state.storage.put('count', count + 1);
    }
    const current = (await this.state.storage.get<number>('count')) || 0;
    return new Response(String(current));
  }
}
