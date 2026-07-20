// SAFE: Unique state per authorization request, persisted and validated

import { Injectable } from '@angular/core';
import { v4 as uuidv4 } from 'uuid';

@Injectable({ providedIn: 'root' })
export class AuthService {
  private readonly stateStore = new Map<string, boolean>();

  getAuthorizationUrl(provider: string): string {
    const state = uuidv4();
    this.stateStore.set(state, true);

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
    if (!this.stateStore.has(state)) return false;
    this.stateStore.delete(state);
    return true;
  }
}
