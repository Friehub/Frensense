// [frensense]
// observation: A React Context provider wraps the entire application with sensitive data such as auth tokens, API keys, or full user records.
// impact: Any consumer of the context — including deeply nested, low-trust components — can access sensitive information. Third-party scripts or compromised components can exfiltrate auth tokens. This violates the principle of least privilege.
// improvement: Provide only the minimum required data through context. Keep sensitive credentials behind a dedicated auth hook that exposes only non-sensitive metadata.

import { createContext, useContext, useState, type ReactNode } from 'react';

interface AuthContextValue {
  token: string;
  user: { id: string; email: string; role: string };
  apiKey: string;
}

const AuthContext = createContext<AuthContextValue | null>(null);

export function AuthProvider({ children }: { children: ReactNode }) {
  const [session] = useState({
    token: 'eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIxMjM0NTY3ODkwIn0',
    user: { id: 'user_1', email: 'admin@example.com', role: 'admin' },
    apiKey: 'sk-proj-abc123def456',
  });

  return (
    <AuthContext.Provider value={session}>
      {children}
    </AuthContext.Provider>
  );
}

export function useAuth() {
  return useContext(AuthContext)!;
}

export function App({ children }: { children: ReactNode }) {
  return <AuthProvider>{children}</AuthProvider>;
}
