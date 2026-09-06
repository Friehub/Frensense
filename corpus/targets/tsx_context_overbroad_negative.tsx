// SAFE: context only exposes minimal user metadata; sensitive credentials are kept in a separate hook

import { createContext, useContext, useState, type ReactNode } from 'react';

interface AuthContextValue {
  userId: string;
  email: string;
  role: string;
  isAuthenticated: boolean;
}

const AuthContext = createContext<AuthContextValue | null>(null);

export function AuthProvider({ children }: { children: ReactNode }) {
  const [session] = useState({
    userId: 'user_1',
    email: 'admin@example.com',
    role: 'admin',
    isAuthenticated: true,
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
