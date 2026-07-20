// SAFE: ID token verified via backend endpoint

import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { firstValueFrom } from 'rxjs';

@Injectable({ providedIn: 'root' })
export class OidcService {
  constructor(private http: HttpClient) {}

  async handleIdToken(idToken: string) {
    return firstValueFrom(
      this.http.post('/api/auth/verify-id-token', { idToken })
    );
  }
}
