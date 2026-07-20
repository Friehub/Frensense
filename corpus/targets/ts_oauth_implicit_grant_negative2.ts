// SAFE: Server-side authorization code flow with PKCE

import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { firstValueFrom } from 'rxjs';

@Injectable({ providedIn: 'root' })
export class ServerAuthService {
  constructor(private http: HttpClient) {}

  async exchangeCode(code: string, verifier: string) {
    return firstValueFrom(
      this.http.post('/api/auth/exchange', { code, code_verifier: verifier })
    );
  }
}
