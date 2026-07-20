// SAFE: Crypto.randomUUID for state with TTL validation

import { Injectable } from '@angular/core';

interface StateEntry {
  createdAt: number;
  used: boolean;
}

@Injectable({ providedIn: 'root' })
export class AuthService {
  private readonly states = new Map<string, StateEntry>();
  private readonly ttl = 5 * 60 * 1000;

  getAuthorizationUrl(provider: string): string {
    const state = crypto.randomUUID();
    this.states.set(state, { createdAt: Date.now(), used: false });

    const params = new URLSearchParams({
      response_type: 'code',
      client_id: 'my-client',
      redirect_uri: 'https://app.example.com/callback',
      state,
      scope: 'openid email',
    });
    return `https://${provider}.example.com/authorize?${params}`;
  }

  handleCallback(code: string, state: string): boolean {
    const entry = this.states.get(state);
    if (!entry || entry.used) return false;
    if (Date.now() - entry.createdAt > this.ttl) return false;
    entry.used = true;
    return true;
  }
}
