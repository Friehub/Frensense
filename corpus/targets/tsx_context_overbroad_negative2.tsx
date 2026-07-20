// SAFE: no sensitive data in context at all — uses a dedicated module for auth tokens

import { createContext, useContext, type ReactNode } from 'react';

function getAuthToken(): string {
  return sessionStorage.getItem('auth_token') ?? '';
}

function getUserInfo() {
  const raw = sessionStorage.getItem('user_info');
  return raw ? JSON.parse(raw) : null;
}

export function Header() {
  const token = getAuthToken();
  return <div>Authenticated: {token ? 'yes' : 'no'}</div>;
}
